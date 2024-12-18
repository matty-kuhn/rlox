use anyhow::{bail, Result};

use crate::{
    ast::{Bin, Expr, Lit, Un},
    err_msg,
    scanner::TokenInfo,
    tokens::{Token, TokenType},
};

struct Parser {
    cursor: usize,
    tokens: TokenInfo,
}

impl Parser {
    pub fn new(tokens: TokenInfo) -> Self {
        Self { cursor: 0, tokens }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    fn synchronize(&mut self) {
        self.advance();
        while let Some(curr_tag) = self.tokens.tags.get(self.cursor) {
            if curr_tag == &TokenType::Semicolon
                || curr_tag == &TokenType::Class
                || curr_tag == &TokenType::Fun
                || curr_tag == &TokenType::Var
                || curr_tag == &TokenType::For
                || curr_tag == &TokenType::If
                || curr_tag == &TokenType::While
                || curr_tag == &TokenType::Print
                || curr_tag == &TokenType::Return
            {
                return;
            }
            self.advance();
        }
    }

    fn advance(&mut self) -> &Token {
        if let Some(tok) = self.tokens.tokens.get(self.cursor) {
            self.cursor += 1;
            return tok;
        }
        self.tokens
            .tokens
            .get(self.cursor - 1)
            .expect("should not yeat have reached end")
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.cursor < self.tokens.tokens.len() {
            if !self.tokens.tags[self.cursor].is_equality() {
                break;
            }
            let curr = self.cursor;
            self.advance();
            // we have an equality sign
            let right = self.comparison()?;
            expr = Expr::Binary(Bin {
                left: expr.into(),
                op: (&self.tokens.tags[curr]).into(),
                right: right.into(),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.cursor < self.tokens.tokens.len() {
            if !self.tokens.tags[self.cursor].is_comp() {
                break;
            }
            let curr = self.cursor;
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary(Bin {
                left: expr.into(),
                op: (&self.tokens.tags[curr]).into(),
                right: right.into(),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.cursor < self.tokens.tokens.len() {
            if !self.tokens.tags[self.cursor].is_term() {
                break;
            }
            let curr = self.cursor;
            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary(Bin {
                left: expr.into(),
                op: (&self.tokens.tags[curr]).into(),
                right: right.into(),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.cursor < self.tokens.tokens.len() {
            if !self.tokens.tags[self.cursor].is_factor() {
                break;
            }
            let curr = self.cursor;
            self.advance();
            let right = self.unary()?;
            expr = Expr::Binary(Bin {
                left: expr.into(),
                op: (&self.tokens.tags[curr]).into(),
                right: right.into(),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        while self.cursor < self.tokens.tokens.len() {
            let curr_tag = self.tokens.tags[self.cursor];
            if !curr_tag.is_unary() {
                break;
            }
            self.advance();
            let right = self.unary()?;
            return Ok(Expr::Unary(match curr_tag {
                TokenType::Minus => Un::Minus(right.into()),
                TokenType::Bang => Un::Bang(right.into()),
                _ => panic!("invalid state: checked tok tag is ! or -"),
            }));
        }

        // self.cursor here is whatever was determined to not be ! or -
        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr> {
        self.advance();
        match self.tokens.tags[self.cursor] {
            TokenType::Nil => Ok(Expr::Literal(Lit::Nil)),
            TokenType::False => Ok(Expr::Literal(Lit::False)),
            TokenType::True => Ok(Expr::Literal(Lit::True)),
            TokenType::Number => Ok(Expr::Literal(Lit::Num(
                self.tokens.tokens[self.cursor].literal.clone(),
            ))),
            TokenType::String => Ok(Expr::Literal(Lit::Str(
                self.tokens.tokens[self.cursor].literal.clone(),
            ))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume_next(TokenType::RightParen, "expected \")\" to close expression")?;
                return Ok(Expr::Grouping(expr.into()));
            }
            _ => panic!("invalid primary sequence"),
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn consume_next(&mut self, tok_type: TokenType, err_ctx: &str) -> Result<&Token> {
        if self.cursor >= self.tokens.tokens.len() {
            bail!("unexpected EOF");
        }

        if self.tokens.tags[self.cursor] == tok_type {
            return Ok(self.advance());
        }
        let err_msg = err_msg!(
            self.tokens.line_nrs[self.cursor],
            err_ctx,
            self.tokens.start_cols[self.cursor]
        );

        eprintln!("{err_msg}");

        bail!(err_msg)
    }
}
