use std::fmt::Display;

use crate::{
    token::{Literal, Token},
    util::AstPrinter,
};

#[derive(Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },

    Grouping {
        expression: Box<Expression>,
    },

    Literal {
        value: Literal,
    },

    Unary {
        operator: Token,
        right: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Expression::format_ast(self))
    }
}
