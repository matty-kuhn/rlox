use crate::{
    err_msg,
    tokens::{Token, TokenType},
};
use anyhow::{anyhow, Error};
use ctx::ScannerCtx;

pub(crate) mod ctx;

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
    pub fn is_finished(&self) -> bool {
        self.ctx.cursor == self.code.len()
    }

    pub fn curr_line(&self) -> usize {
        self.ctx.curr_line.into()
    }

    pub fn curr_col(&self) -> usize {
        self.ctx.curr_col
    }

    pub fn has_errors(&self) -> bool {
        !self.ctx.errors.is_empty()
    }

    pub fn errors(&self) -> &[anyhow::Error] {
        &self.ctx.errors
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.peek(1);
        if next.is_none() {
            if self.ctx.cursor == self.code.len() - 1 {
                self.ctx.cursor += 1
            }
            return next;
        }
        self.ctx.cursor += 1;
        self.ctx.curr_col += 1;
        next
    }

    fn rewind(&mut self) {
        self.ctx.cursor -= 1;
        self.ctx.curr_col -= 1;
    }

    fn peek(&self, n: usize) -> Option<char> {
        self.code.chars().nth(self.ctx.cursor + n)
    }

    fn string_started(&mut self) -> Option<(TokenType, Token)> {
        let start = (self.ctx.curr_line, self.ctx.curr_col);
        let mut builder = String::new();
        while let Some(curr_char) = self.advance() {
            if curr_char == '"' {
                return Some((TokenType::String, Token::new(&builder, true)));
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

    fn want_number(&mut self) -> (TokenType, Token) {
        let mut builder = String::new();
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

        // rewind by 1
        self.rewind();
        (TokenType::Number, Token::new(&builder, true))
    }

    fn want_ident(&mut self) -> (TokenType, Token) {
        let mut builder = String::new();
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
        // rewind by 1
        self.rewind();
        (tok_type, Token::new(&builder, false))
    }

    fn get_next_token(&mut self) -> Option<(TokenType, Token)> {
        while let Some(curr_char) = self.peek(0) {
            match curr_char {
                '"' => return self.string_started(),
                '(' => {
                    return Some((
                        TokenType::LeftParen,
                        Token::new(&format!("{curr_char}"), false),
                    ))
                }
                ')' => {
                    return Some((
                        TokenType::RightParen,
                        Token::new(&format!("{curr_char}"), false),
                    ))
                }
                '{' => {
                    return Some((
                        TokenType::LeftBrace,
                        Token::new(&format!("{curr_char}"), false),
                    ))
                }
                '}' => {
                    return Some((
                        TokenType::RightBrace,
                        Token::new(&format!("{curr_char}"), false),
                    ))
                }
                ',' => return Some((TokenType::Comma, Token::new(&format!("{curr_char}"), false))),
                '.' => return Some((TokenType::Dot, Token::new(&format!("{curr_char}"), false))),
                '-' => return Some((TokenType::Minus, Token::new(&format!("{curr_char}"), false))),
                '+' => return Some((TokenType::Plus, Token::new(&format!("{curr_char}"), false))),
                ';' => {
                    return Some((
                        TokenType::Semicolon,
                        Token::new(&format!("{curr_char}"), false),
                    ))
                }
                '*' => return Some((TokenType::Star, Token::new(&format!("{curr_char}"), false))),
                num if num >= '0' && num <= '9' => return Some(self.want_number()),
                '!' => {
                    let Some(next) = self.peek(1) else {
                        return Some((TokenType::Bang, Token::new(&format!("{curr_char}"), false)));
                    };
                    if next == '=' {
                        let next = self.advance().unwrap();
                        return Some((
                            TokenType::BangEqual,
                            Token::new(&format!("{curr_char}{next}"), false),
                        ));
                    } else {
                        return Some((TokenType::Bang, Token::new(&format!("{curr_char}"), false)));
                    }
                }
                '=' => {
                    let Some(next) = self.peek(1) else {
                        return Some((
                            TokenType::Equal,
                            Token::new(&format!("{curr_char}"), false),
                        ));
                    };
                    if next == '=' {
                        let next = self.advance().unwrap();
                        return Some((
                            TokenType::EqualEqual,
                            Token::new(&format!("{curr_char}{next}"), false),
                        ));
                    } else {
                        return Some((
                            TokenType::Equal,
                            Token::new(&format!("{curr_char}"), false),
                        ));
                    }
                }
                '>' => {
                    let Some(next) = self.peek(0) else {
                        return Some((
                            TokenType::Greater,
                            Token::new(&format!("{curr_char}"), false),
                        ));
                    };
                    if next == '=' {
                        let next = self.advance().unwrap();
                        return Some((
                            TokenType::GreaterEqual,
                            Token::new(&format!("{curr_char}{next}"), false),
                        ));
                    } else {
                        return Some((
                            TokenType::Greater,
                            Token::new(&format!("{curr_char}"), false),
                        ));
                    }
                }
                '<' => {
                    let Some(next) = self.peek(1) else {
                        return Some((TokenType::Less, Token::new(&format!("{curr_char}"), false)));
                    };
                    if next == '=' {
                        let next = self.advance().unwrap();
                        return Some((
                            TokenType::LessEqual,
                            Token::new(&format!("{curr_char}{next}"), false),
                        ));
                    } else {
                        return Some((TokenType::Less, Token::new(&format!("{curr_char}"), false)));
                    }
                }
                '/' => {
                    if let Some(next) = self.peek(1) {
                        if next == '/' {
                            // spin until end of line
                            while let Some(stuff) = self.advance() {
                                if stuff == '\n' {
                                    self.advance();
                                    self.ctx.newline();
                                    break;
                                }
                            }
                        } else {
                            return Some((
                                TokenType::Slash,
                                Token::new(&format!("{curr_char}"), false),
                            ));
                        }
                    } else {
                        return Some((
                            TokenType::Slash,
                            Token::new(&format!("{curr_char}"), false),
                        ));
                    }
                }
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.ctx.newline();
                }
                _ => {
                    return Some(self.want_ident());
                }
            }
        }

        Some((TokenType::Eof, Token::new("", false)))
    }

    fn error(&mut self, err: &str) {
        self.ctx.errors.push(anyhow!(err_msg!(
            self.ctx.curr_line,
            self.ctx.curr_col,
            err
        )));
    }

    pub(crate) fn run(mut self) -> TokenInfo {
        let mut tokens = vec![];
        let mut tags = vec![];
        let mut line_nrs = vec![];
        let mut end_cols = vec![];
        loop {
            let Some((tag, tok)) = self.get_next_token() else {
                debug_assert!(tags[tags.len() - 1] == TokenType::Eof);
                return TokenInfo {
                    tokens,
                    tags,
                    line_nrs,
                    end_cols,
                    errors: self.ctx.errors,
                };
            };
            tags.push(tag);
            tokens.push(tok);
            line_nrs.push(self.ctx.curr_line);
            end_cols.push(self.ctx.curr_col);
            if tag == TokenType::Eof {
                break;
            }
            self.advance();
        }
        debug_assert!(tags[tags.len() - 1] == TokenType::Eof);
        TokenInfo {
            tokens,
            tags,
            line_nrs,
            end_cols,
            errors: self.ctx.errors,
        }
    }
}

#[derive(Debug)]
pub(crate) struct TokenInfo {
    pub(crate) tokens: Vec<Token>,
    pub(crate) tags: Vec<TokenType>,
    pub(crate) line_nrs: Vec<usize>,
    pub(crate) end_cols: Vec<usize>,
    pub(crate) errors: Vec<Error>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_symbols() {
        let code = r#"// this is a comment
(( )){} // grouping stuff
!*+-/=<> <= ==!= // operators"#;
        let exp = TokenInfo {
            tokens: vec![
                Token::new("(", false),
                Token::new("(", false),
                Token::new(")", false),
                Token::new(")", false),
                Token::new("{", false),
                Token::new("}", false),
                Token::new("!", false),
                Token::new("*", false),
                Token::new("+", false),
                Token::new("-", false),
                Token::new("/", false),
                Token::new("=", false),
                Token::new("<", false),
                Token::new(">", false),
                Token::new("<=", false),
                Token::new("==", false),
                Token::new("!=", false),
                Token::new("", false),
            ],
            tags: vec![
                TokenType::LeftParen,
                TokenType::LeftParen,
                TokenType::RightParen,
                TokenType::RightParen,
                TokenType::LeftBrace,
                TokenType::RightBrace,
                TokenType::Bang,
                TokenType::Star,
                TokenType::Plus,
                TokenType::Minus,
                TokenType::Slash,
                TokenType::Equal,
                TokenType::Less,
                TokenType::Greater,
                TokenType::LessEqual,
                TokenType::EqualEqual,
                TokenType::BangEqual,
                TokenType::Eof,
            ],
            line_nrs: vec![2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
            end_cols: vec![0, 1, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 7, 10, 13, 15, 28],
            errors: vec![],
        };
        let scan_res = Scanner::new(code).run();

        assert_eq!(scan_res.tokens, exp.tokens);
        assert_eq!(scan_res.tags, exp.tags);
        assert_eq!(scan_res.end_cols, exp.end_cols);
        assert_eq!(scan_res.line_nrs, exp.line_nrs);
        assert!(scan_res.errors.is_empty());
    }

    #[test]
    fn test_scan_simple_numbers() {
        let code = r#"123 123.456 0.123"#;
        let exp = TokenInfo {
            tokens: vec![
                Token::new("123", true),
                Token::new("123.456", true),
                Token::new("0.123", true),
                Token::new("", false),
            ],
            tags: vec![
                TokenType::Number,
                TokenType::Number,
                TokenType::Number,
                TokenType::Eof,
            ],
            line_nrs: vec![1, 1, 1, 1],
            end_cols: vec![2, 10, 15, 15],
            errors: vec![],
        };
        let scan_res = Scanner::new(code).run();

        assert_eq!(scan_res.tokens, exp.tokens);
        assert_eq!(scan_res.tags, exp.tags);
        assert_eq!(scan_res.end_cols, exp.end_cols);
        assert_eq!(scan_res.line_nrs, exp.line_nrs);
        assert!(scan_res.errors.is_empty());
    }

    #[test]
    fn test_simple_idents() {
        let code = r#"foo bar baz if and fun else or nil"#;
        let exp = TokenInfo {
            tokens: vec![
                Token::new("foo", false),
                Token::new("bar", false),
                Token::new("baz", false),
                Token::new("if", false),
                Token::new("and", false),
                Token::new("fun", false),
                Token::new("else", false),
                Token::new("or", false),
                Token::new("nil", false),
                Token::new("", false),
            ],
            tags: vec![
                TokenType::Identifier,
                TokenType::Identifier,
                TokenType::Identifier,
                TokenType::If,
                TokenType::And,
                TokenType::Fun,
                TokenType::Else,
                TokenType::Or,
                TokenType::Nil,
                TokenType::Eof,
            ],
            line_nrs: vec![1; 10],
            end_cols: vec![2, 6, 10, 13, 17, 21, 26, 29, 32, 32],
            errors: vec![],
        };
        let scan_res = Scanner::new(code).run();

        assert_eq!(scan_res.tokens, exp.tokens);
        assert_eq!(scan_res.tags, exp.tags);
        assert_eq!(scan_res.end_cols, exp.end_cols);
        assert_eq!(scan_res.line_nrs, exp.line_nrs);
        assert!(scan_res.errors.is_empty());
    }
}
