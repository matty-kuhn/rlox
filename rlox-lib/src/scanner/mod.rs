use crate::err_msg;
use anyhow::{anyhow, bail, Result};
use ctx::ScannerCtx;
use std::{iter::Peekable, str::Chars};
use token::{Token, TokenType};

pub(crate) mod ctx;
pub(crate) mod token;

pub(crate) struct Scanner<'code> {
    code: Peekable<Chars<'code>>,
    ctx: ScannerCtx,
}

impl<'code> Scanner<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            code: code.chars().peekable(),
            ctx: ScannerCtx::new(),
        }
    }
}

/// Functions available in any state
impl<'code> Scanner<'code> {
    pub fn has_errors(&self) -> bool {
        !self.ctx.errors.is_empty()
    }

    pub fn errors(&self) -> &[anyhow::Error] {
        &self.ctx.errors
    }
    fn string_started(&mut self) -> Option<Token> {
        let start = (self.ctx.curr_line, self.ctx.curr_col);
        let mut builder = String::new();
        while let Some(curr_char) = self.code.next() {
            if curr_char == '"' {
                return Some(Token::new(TokenType::String, &builder));
            }
            builder.push(curr_char);
        }
        self.error(&format!(
            "unterminated string starting at: {}:{}",
            start.0, start.1
        ));
        None
    }

    fn want_number(&mut self, first: char) -> Option<Token> {
        let mut builder = String::new();
        todo!()

        // self.error(&format!(
        //     "invalid character found in number: ",
        // ));
        // None
    }

    fn get_next_token(&mut self) -> Option<Token> {
        while let Some(curr_char) = self.code.next() {
            match curr_char {
                '"' => return self.string_started(),
                '(' => return Some(Token::new(TokenType::LeftParen, &format!("{curr_char}"))),
                ')' => return Some(Token::new(TokenType::RightParen, &format!("{curr_char}"))),
                '{' => return Some(Token::new(TokenType::LeftBrace, &format!("{curr_char}"))),
                '}' => return Some(Token::new(TokenType::RightBrace, &format!("{curr_char}"))),
                ',' => return Some(Token::new(TokenType::Comma, &format!("{curr_char}"))),
                '.' => return Some(Token::new(TokenType::Dot, &format!("{curr_char}"))),
                '-' => return Some(Token::new(TokenType::Minus, &format!("{curr_char}"))),
                '+' => return Some(Token::new(TokenType::Plus, &format!("{curr_char}"))),
                ';' => return Some(Token::new(TokenType::Semicolon, &format!("{curr_char}"))),
                '*' => return Some(Token::new(TokenType::Star, &format!("{curr_char}"))),
                num if num >= '0' && num <= '9' => return self.want_number(num),
                '!' => {
                    let Some(next) = self.code.peek() else {
                        return Some(Token::new(TokenType::Bang, &format!("{curr_char}")));
                    };
                    if next == &'=' {
                        let next = self.code.next().unwrap();
                        return Some(Token::new(
                            TokenType::BangEqual,
                            &format!("{curr_char}{next}"),
                        ));
                    } else {
                        return Some(Token::new(TokenType::Bang, &format!("{curr_char}")));
                    }
                }
                '=' => {
                    let Some(next) = self.code.peek() else {
                        return Some(Token::new(TokenType::Equal, &format!("{curr_char}")));
                    };
                    if next == &'=' {
                        let next = self.code.next().unwrap();
                        return Some(Token::new(
                            TokenType::EqualEqual,
                            &format!("{curr_char}{next}"),
                        ));
                    } else {
                        return Some(Token::new(TokenType::Equal, &format!("{curr_char}")));
                    }
                }
                '>' => {
                    let Some(next) = self.code.peek() else {
                        return Some(Token::new(TokenType::Greater, &format!("{curr_char}")));
                    };
                    if next == &'=' {
                        let next = self.code.next().unwrap();
                        return Some(Token::new(
                            TokenType::GreaterEqual,
                            &format!("{curr_char}{next}"),
                        ));
                    } else {
                        return Some(Token::new(TokenType::Greater, &format!("{curr_char}")));
                    }
                }
                '<' => {
                    let Some(next) = self.code.peek() else {
                        return Some(Token::new(TokenType::Less, &format!("{curr_char}")));
                    };
                    if next == &'=' {
                        let next = self.code.next().unwrap();
                        return Some(Token::new(
                            TokenType::LessEqual,
                            &format!("{curr_char}{next}"),
                        ));
                    } else {
                        return Some(Token::new(TokenType::Less, &format!("{curr_char}")));
                    }
                }
                '/' => {
                    if let Some(next) = self.code.peek() {
                        if next == &'/' {
                            // spin until end of line
                            while let Some(stuff) = self.code.next() {
                                if stuff == '\n' {
                                    break;
                                }
                            }
                        } else {
                            return Some(Token::new(TokenType::Slash, &format!("{curr_char}")));
                        }
                    } else {
                        return Some(Token::new(TokenType::Slash, &format!("{curr_char}")));
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => {
                    self.ctx.newline();
                }
                x => {
                    self.error(&format!("unrecognized token: {x}"));
                }
            }
        }

        None
    }

    fn error(&mut self, err: &str) {
        self.ctx.errors.push(anyhow!(err_msg!(
            self.ctx.curr_line,
            self.ctx.curr_col,
            err
        )));
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // basic peek to see if there are enough chars left to look at
        if self.code.peek().is_none() {
            None
        } else {
            self.get_next_token()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_symbols() {
        let code = r#"// this is a comment
(( )){} // grouping stuff
!*+-/=<> <= == // operators"#;
        let exp = vec![
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::RightBrace, "}"),
            Token::new(TokenType::Bang, "!"),
            Token::new(TokenType::Star, "*"),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::Minus, "-"),
            Token::new(TokenType::Slash, "/"),
            Token::new(TokenType::Equal, "="),
            Token::new(TokenType::Less, "<"),
            Token::new(TokenType::Greater, ">"),
            Token::new(TokenType::LessEqual, "<="),
            Token::new(TokenType::EqualEqual, "=="),
        ];
        let mut scanner = Scanner::new(code).into_iter();

        let mut idx = 0;
        while let Some(token) = scanner.next() {
            assert_eq!(exp[idx], token);
            idx += 1;
        }
    }
}
