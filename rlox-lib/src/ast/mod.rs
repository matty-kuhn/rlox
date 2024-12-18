//! Our Grammar
//! expression     → literal
//!                | unary
//!                | binary
//!                | grouping ;
//!
//! literal        → NUMBER | STRING | "true" | "false" | "nil" ;
//! grouping       → "(" expression ")" ;
//! unary          → ( "-" | "!" ) expression ;
//! binary         → expression operator expression ;
//! operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//!                | "+"  | "-"  | "*" | "/" ;

mod printer;
use crate::tokens::{Token, TokenType, Value};
use std::{fmt::Display, rc::Rc};

pub(crate) trait Visitor {
    type Output;
    fn visit_binary(&self, expr: &Bin) -> Self::Output;
    fn visit_unary(&self, expr: &Un) -> Self::Output;
    fn visit_literal(&self, expr: &Lit) -> Self::Output;
    fn visit_grouping(&self, expr: &Rc<Expr>) -> Self::Output;
}
pub(crate) trait VisitorMut {
    type Output;
    fn visit_binary_mut(&mut self, expr: &Bin) -> Self::Output;
    fn visit_unary_mut(&mut self, expr: &Un) -> Self::Output;
    fn visit_literal_mut(&mut self, expr: &Lit) -> Self::Output;
    fn visit_grouping_mut(&mut self, expr: &Rc<Expr>) -> Self::Output;
}

pub(crate) enum Lit {
    True,
    False,
    Nil,
    Num(Value),
    Str(Value),
}

pub(crate) enum Un {
    Minus(Rc<Expr>),
    Bang(Rc<Expr>),
}

impl Un {
    fn inner(&self) -> &Rc<Expr> {
        match self {
            Un::Minus(x) | Un::Bang(x) => x,
        }
    }
}

pub(crate) enum Ops {
    Minus,
    Plus,
    BangEqual,
    Slash,
    Star,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl From<&TokenType> for Ops {
    fn from(value: &TokenType) -> Self {
        match value {
            crate::tokens::TokenType::LeftParen
            | crate::tokens::TokenType::RightParen
            | crate::tokens::TokenType::LeftBrace
            | crate::tokens::TokenType::RightBrace
            | crate::tokens::TokenType::Comma
            | crate::tokens::TokenType::Dot
            | crate::tokens::TokenType::Bang
            | crate::tokens::TokenType::Equal
            | crate::tokens::TokenType::Semicolon
            | crate::tokens::TokenType::Eof
            | crate::tokens::TokenType::Identifier
            | crate::tokens::TokenType::String
            | crate::tokens::TokenType::Number
            | crate::tokens::TokenType::And
            | crate::tokens::TokenType::Class
            | crate::tokens::TokenType::Else
            | crate::tokens::TokenType::False
            | crate::tokens::TokenType::Fun
            | crate::tokens::TokenType::For
            | crate::tokens::TokenType::If
            | crate::tokens::TokenType::Nil
            | crate::tokens::TokenType::Or
            | crate::tokens::TokenType::Print
            | crate::tokens::TokenType::Return
            | crate::tokens::TokenType::Super
            | crate::tokens::TokenType::This
            | crate::tokens::TokenType::True
            | crate::tokens::TokenType::Var
            | crate::tokens::TokenType::While => panic!("invalid operator"),
            crate::tokens::TokenType::Minus => Self::Minus,
            crate::tokens::TokenType::Plus => Self::Plus,
            crate::tokens::TokenType::Slash => Self::Slash,
            crate::tokens::TokenType::Star => Self::Star,
            crate::tokens::TokenType::BangEqual => Self::BangEqual,
            crate::tokens::TokenType::EqualEqual => Self::EqualEqual,
            crate::tokens::TokenType::Greater => Self::Greater,
            crate::tokens::TokenType::GreaterEqual => Self::GreaterEqual,
            crate::tokens::TokenType::Less => Self::Less,
            crate::tokens::TokenType::LessEqual => Self::LessEqual,
        }
    }
}

pub(crate) struct Bin {
    pub(crate) left: Rc<Expr>,
    pub(crate) op: Ops,
    pub(crate) right: Rc<Expr>,
}

pub(crate) enum Expr {
    Literal(Lit),
    Unary(Un),
    Binary(Bin),
    Grouping(Rc<Expr>),
}

impl Expr {
    pub(crate) fn accept<T>(&self, visitor: &T) -> T::Output
    where
        T: Visitor,
    {
        match self {
            Expr::Literal(lit) => visitor.visit_literal(lit),
            Expr::Unary(un) => visitor.visit_unary(un),
            Expr::Binary(bin) => visitor.visit_binary(bin),
            Expr::Grouping(grp) => visitor.visit_grouping(grp),
        }
    }

    pub(crate) fn accept_mut<T>(&self, visitor: &mut T) -> T::Output
    where
        T: VisitorMut,
    {
        match self {
            Expr::Literal(lit) => visitor.visit_literal_mut(lit),
            Expr::Unary(un) => visitor.visit_unary_mut(un),
            Expr::Binary(bin) => visitor.visit_binary_mut(bin),
            Expr::Grouping(grp) => visitor.visit_grouping_mut(grp),
        }
    }
}

impl Display for Ops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ops::Minus => "-",
                Ops::Plus => "+",
                Ops::BangEqual => "!=",
                Ops::Slash => "/",
                Ops::Star => "*",
                Ops::EqualEqual => "==",
                Ops::Greater => ">",
                Ops::GreaterEqual => ">=",
                Ops::Less => "<",
                Ops::LessEqual => "<=",
            }
        )
    }
}
