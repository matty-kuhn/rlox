use crate::err_msg;
use anyhow::{anyhow, Result};
use ctx::{CtxState, Parsing, ScannerCtx};
use token::{Token, TokenType};

pub(crate) mod ctx;
pub(crate) mod token;

pub(crate) struct Scanner<'code, State>
where
    State: CtxState,
{
    code: &'code str,
    ctx: ScannerCtx<State>,
}

/// Functions available in any state
impl<'code, State> Scanner<'code, State>
where
    State: CtxState,
{
    pub fn new(code: &'code str) -> Self {
        Self {
            code,
            ctx: ScannerCtx::new()
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.ctx.errors.is_empty()
    }

    pub fn errors(&self) -> &[anyhow::Error] {
        &self.ctx.errors
    }
}

impl<'code, State> Scanner<'code, State>
where
    State: Parsing,
{
    fn get_next_token(&mut self) -> Result<Token> {
        while let Some(curr_char) = self.code.get(self.ctx.cursor..self.ctx.cursor) {
            self.ctx.cursor += 1;
            self.ctx.curr_col += 1;
            let tag = match curr_char {
                "(" => TokenType::LeftParen,
                ")" => TokenType::RightParen,
                "{" => TokenType::LeftBrace,
                "}" => TokenType::RightBrace,
                "," => TokenType::Comma,
                "." => TokenType::Dot,
                "-" => TokenType::Minus,
                "+" => TokenType::Plus,
                ";" => TokenType::Semicolon,
                "*" => TokenType::Star,
                // "!" => {}
                x => {
                    self.ctx.errors.push(anyhow!(err_msg!(
                        self.ctx.curr_line,
                        self.ctx.curr_col,
                        format!("unrecognized token: {x}")
                    )));
                    continue;
                }
            };
            return Ok(Token::new(tag, curr_char));
        }

        todo!()
    }
}

impl<State> Iterator for Scanner<'_, State>
where
    State: Parsing,
{
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        // basic peek to see if there are enough chars left to look at
        if (self.ctx.cursor as usize) < self.code.len() {
            None
        } else {
            Some(self.get_next_token())
        }
    }
}
