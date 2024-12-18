use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
    // pub(crate) tag: TokenType,
    pub(crate) lexeme: Rc<str>,
    pub(crate) literal: Value,
    // line: usize,
}

impl Token {
    pub(crate) fn new(lexeme: &str, literal: bool) -> Self {
        let rc: Rc<str> = Rc::from(lexeme);
        let literal = if literal {
            if let Ok(num) = lexeme.parse() {
                Value::Num(num)
            } else {
                Value::String(rc.clone())
            }
        } else {
            Value::None
        };
        Self {
            // tag,
            lexeme: rc,
            literal,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    pub(crate) fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenType::And
                | TokenType::Class
                | TokenType::Else
                | TokenType::False
                | TokenType::Fun
                | TokenType::For
                | TokenType::If
                | TokenType::Nil
                | TokenType::Or
                | TokenType::Print
                | TokenType::Return
                | TokenType::Super
                | TokenType::This
                | TokenType::True
                | TokenType::Var
                | TokenType::While
        )
    }

    pub(crate) fn from_str(value: &str) -> Option<Self> {
        match value {
            "(" => Some(TokenType::LeftParen),
            ")" => Some(TokenType::RightParen),
            "{" => Some(TokenType::LeftBrace),
            "}" => Some(TokenType::RightBrace),
            "," => Some(TokenType::Comma),
            "." => Some(TokenType::Dot),
            "-" => Some(TokenType::Minus),
            "+" => Some(TokenType::Plus),
            ";" => Some(TokenType::Semicolon),
            "*" => Some(TokenType::Star),
            "!" => Some(TokenType::Bang),
            "!=" => Some(TokenType::BangEqual),
            "=" => Some(TokenType::Equal),
            "==" => Some(TokenType::EqualEqual),
            ">" => Some(TokenType::Greater),
            ">=" => Some(TokenType::GreaterEqual),
            "<" => Some(TokenType::Less),
            "<=" => Some(TokenType::LessEqual),
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
    pub(crate) fn is_equality(&self) -> bool {
        matches!(self, TokenType::EqualEqual) || matches!(self, TokenType::BangEqual)
    }

    pub(crate) fn is_comp(&self) -> bool {
        matches!(self, TokenType::Greater)
            || matches!(self, TokenType::GreaterEqual)
            || matches!(self, TokenType::Less)
            || matches!(self, TokenType::LessEqual)
    }

    pub(crate) fn is_term(&self) -> bool {
        matches!(self, TokenType::Minus) || matches!(self, TokenType::Plus)
    }
    pub(crate) fn is_factor(&self) -> bool {
        matches!(self, TokenType::Slash) || matches!(self, TokenType::Star)
    }
    pub(crate) fn is_unary(&self) -> bool {
        matches!(self, TokenType::Bang) || matches!(self, TokenType::Minus)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::String => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),
            TokenType::And => write!(f, "And"),
            TokenType::Class => write!(f, "Class"),
            TokenType::Else => write!(f, "Else"),
            TokenType::False => write!(f, "False"),
            TokenType::Fun => write!(f, "Fun"),
            TokenType::For => write!(f, "For"),
            TokenType::If => write!(f, "If"),
            TokenType::Nil => write!(f, "Nil"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Super => write!(f, "Super"),
            TokenType::This => write!(f, "This"),
            TokenType::True => write!(f, "True"),
            TokenType::Var => write!(f, "Var"),
            TokenType::While => write!(f, "While"),
            TokenType::Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    String(Rc<str>),
    Num(f64),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{s}\""),
            Value::Num(num) => write!(f, "{num}"),
            Value::None => write!(f, "nil"),
        }
    }
}
