use crate::token::{Literal, Token};

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
