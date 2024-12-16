use anyhow::{bail, Result};

use crate::{
    ast::{Bin, Expr, Lit, Un},
    err_msg,
    scanner::Scanner,
    tokens::{Token, TokenType},
};

struct Parser<'code> {
    cursor: usize,
    scanner: Scanner<'code>,
}

impl<'code> Parser<'code> {
    pub fn new(scanner: Scanner<'code>) -> Self {
        Self { cursor: 0, scanner }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    fn synchronize(&mut self) {
        self.advance();
        while let Some(curr) = self.scanner.nth(self.cursor) {
            if curr.tag == TokenType::Semicolon
                || curr.tag == TokenType::Class
                || curr.tag == TokenType::Fun
                || curr.tag == TokenType::Var
                || curr.tag == TokenType::For
                || curr.tag == TokenType::If
                || curr.tag == TokenType::While
                || curr.tag == TokenType::Print
                || curr.tag == TokenType::Return
            {
                return;
            }
            self.advance();
        }
    }

    fn advance(&mut self) -> Token {
        if let Some(tok) = self.scanner.nth(self.cursor) {
            self.cursor += 1;
            return tok;
        }
        return self
            .scanner
            .nth(self.cursor - 1)
            .expect("should not yeat have reached end");
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while let Some(tok) = self.scanner.nth(self.cursor) {
            if !tok.is_equality() {
                break;
            }
            self.advance();
            // we have an equality sign
            let right = self.comparison()?;
            expr = Expr::Binary(Bin {
                left: expr.into(),
                op: (&tok).into(),
                right: right.into(),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while let Some(tok) = self.scanner.nth(self.cursor) {
            if !tok.is_comp() {
                break;
            }
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary(Bin {
                left: expr.into(),
                op: (&tok).into(),
                right: right.into(),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while let Some(tok) = self.scanner.nth(self.cursor) {
            if !tok.is_term() {
                break;
            }
            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary(Bin {
                left: expr.into(),
                op: (&tok).into(),
                right: right.into(),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while let Some(tok) = self.scanner.nth(self.cursor) {
            if !tok.is_factor() {
                break;
            }
            self.advance();
            let right = self.unary()?;
            expr = Expr::Binary(Bin {
                left: expr.into(),
                op: (&tok).into(),
                right: right.into(),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        while let Some(tok) = self.scanner.nth(self.cursor) {
            if !tok.is_unary() {
                break;
            }
            self.advance();
            let right = self.unary()?;
            return Ok(Expr::Unary(match tok.tag {
                TokenType::Minus => Un::Minus(right.into()),
                TokenType::Bang => Un::Bang(right.into()),
                _ => panic!("invalid state: checked tok tag is ! or -"),
            }));
        }

        // self.cursor here is whatever was determined to not be ! or -
        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr> {
        let curr = self.advance();
        match curr.tag {
            TokenType::Nil => Ok(Expr::Literal(Lit::Nil)),
            TokenType::False => Ok(Expr::Literal(Lit::False)),
            TokenType::True => Ok(Expr::Literal(Lit::True)),
            TokenType::Number => Ok(Expr::Literal(Lit::Num(curr.literal))),
            TokenType::String => Ok(Expr::Literal(Lit::Str(curr.literal))),
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

    fn consume_next(&mut self, tok_type: TokenType, err_ctx: &str) -> Result<Token> {
        let Some(peek) = self.scanner.nth(self.cursor) else {
            bail!(format!("reached EOF, {err_ctx}"));
        };

        if peek.tag == tok_type {
            return Ok(self.advance());
        }
        let err_msg = err_msg!(self.scanner.curr_line(), err_ctx, self.scanner.curr_col());

        eprintln!("{err_msg}");

        bail!(err_msg)
    }
}
