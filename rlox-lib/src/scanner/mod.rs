use crate::err_msg;
use anyhow::anyhow;
use ctx::ScannerCtx;
use token::{Token, TokenType};

pub(crate) mod ctx;
pub(crate) mod token;

pub(crate) struct Scanner<'code> {
    code: &'code str,
    ctx: ScannerCtx,
}

impl<'code> Scanner<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            code,
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

    fn advance(&mut self) -> Option<char> {
        let next = self.code.chars().nth(self.ctx.cursor);
        if next.is_none() {
            return None;
        }
        self.ctx.cursor += 1;
        if next == Some('\n') {
            self.ctx.newline();
        }
        next
    }

    fn peek(&self, n: usize) -> Option<char> {
        self.code.chars().nth(self.ctx.cursor + n)
    }

    fn string_started(&mut self) -> Option<Token> {
        let start = (self.ctx.curr_line, self.ctx.curr_col);
        let mut builder = String::new();
        while let Some(curr_char) = self.advance() {
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

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn want_number(&mut self, first: char) -> Token {
        let mut builder = String::new();
        builder.push(first);
        while let Some(next) = self.peek(0) {
            if next.is_digit(10) {
                builder.push(next);
                self.advance();
            } else if next == '.' {
                if let Some(next_next) = self.peek(1) {
                    if next_next.is_digit(10) {
                        builder.push(next);
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Token::new(TokenType::Number, &builder)
    }

    fn want_ident(&mut self, first: char) -> Token {
        let mut builder = String::new();
        builder.push(first);
        while let Some(next) = self.peek(0) {
            if Self::is_alpha(next) {
                builder.push(next);
                self.advance();
            } else {
                break;
            }
        }
        let tok_type = if let Some(tok) = TokenType::from_str(&builder) {
            tok
        } else {
            TokenType::Identifier
        };
        Token::new(TokenType::Identifier, &builder)
    }

    fn get_next_token(&mut self) -> Option<Token> {
        while let Some(curr_char) = self.advance() {
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
                num if num >= '0' && num <= '9' => return Some(self.want_number(num)),
                '!' => {
                    let Some(next) = self.peek(0) else {
                        return Some(Token::new(TokenType::Bang, &format!("{curr_char}")));
                    };
                    if next == '=' {
                        let next = self.advance().unwrap();
                        return Some(Token::new(
                            TokenType::BangEqual,
                            &format!("{curr_char}{next}"),
                        ));
                    } else {
                        return Some(Token::new(TokenType::Bang, &format!("{curr_char}")));
                    }
                }
                '=' => {
                    let Some(next) = self.peek(0) else {
                        return Some(Token::new(TokenType::Equal, &format!("{curr_char}")));
                    };
                    if next == '=' {
                        let next = self.advance().unwrap();
                        return Some(Token::new(
                            TokenType::EqualEqual,
                            &format!("{curr_char}{next}"),
                        ));
                    } else {
                        return Some(Token::new(TokenType::Equal, &format!("{curr_char}")));
                    }
                }
                '>' => {
                    let Some(next) = self.peek(0) else {
                        return Some(Token::new(TokenType::Greater, &format!("{curr_char}")));
                    };
                    if next == '=' {
                        let next = self.advance().unwrap();
                        return Some(Token::new(
                            TokenType::GreaterEqual,
                            &format!("{curr_char}{next}"),
                        ));
                    } else {
                        return Some(Token::new(TokenType::Greater, &format!("{curr_char}")));
                    }
                }
                '<' => {
                    let Some(next) = self.peek(0) else {
                        return Some(Token::new(TokenType::Less, &format!("{curr_char}")));
                    };
                    if next == '=' {
                        let next = self.advance().unwrap();
                        return Some(Token::new(
                            TokenType::LessEqual,
                            &format!("{curr_char}{next}"),
                        ));
                    } else {
                        return Some(Token::new(TokenType::Less, &format!("{curr_char}")));
                    }
                }
                '/' => {
                    if let Some(next) = self.peek(0) {
                        if next == '/' {
                            // spin until end of line
                            while let Some(stuff) = self.advance() {
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
                    self.want_ident(x);
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
        if self.peek(0).is_none() {
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
        while let Some(token) = scanner.get_next_token() {
            assert_eq!(exp[idx], token);
            idx += 1;
        }
    }

    #[test]
    fn test_simple_numbers() {
        let code = r#"123 123.456 0.123"#;
        let exp = vec![
            Token::new(TokenType::Number, "123"),
            Token::new(TokenType::Number, "123.456"),
            Token::new(TokenType::Number, "0.123"),
        ];
        let mut scanner = Scanner::new(code).into_iter();
        let mut idx = 0;
        while let Some(token) = scanner.get_next_token() {
            assert_eq!(exp[idx], token);
            idx += 1;
        }
    }

    #[test]
    fn test_simple_idents() {
        let code = r#"foo bar baz if and fun else or nil"#;
        let exp = vec![
            Token::new(TokenType::Identifier, "foo"),
            Token::new(TokenType::Identifier, "bar"),
            Token::new(TokenType::Identifier, "baz"),
            Token::new(TokenType::If, "if"),
            Token::new(TokenType::And, "and"),
            Token::new(TokenType::Fun, "fun"),
            Token::new(TokenType::Else, "else"),
            Token::new(TokenType::Or, "or"),
            Token::new(TokenType::Nil, "nil"),
        ];
        let mut scanner = Scanner::new(code).into_iter();
        let mut idx = 0;
        while let Some(token) = scanner.get_next_token() {
            assert_eq!(exp[idx], token);
            idx += 1;
        }
    }
}
