use std::fmt::Display;

use crate::expression::Expression;
use crate::token::{Literal, TokenType};

struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expression: Expression) -> Value {
        match expression {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left);
                let right = self.evaluate(*right);

                let left_num = match left {
                    Value::Number(num) => num,
                    _ => unreachable!("Left part of binary expression must be a number"),
                };

                let right_num = match right {
                    Value::Number(num) => num,
                    _ => unreachable!("right part of binary expression must be a number"),
                };

                match operator.token_type {
                    // Arithmetic
                    TokenType::Minus => Value::Number(left_num - right_num),
                    TokenType::Slash => Value::Number(left_num / right_num),
                    TokenType::Star => Value::Number(left_num * right_num),
                    TokenType::Plus => match (&left, &right) {
                        (Value::Number(left_num), Value::Number(right_num)) => {
                            Value::Number(left_num + right_num)
                        }
                        (Value::String(left_str), Value::String(right_str)) => {
                            Value::String(format!("{}{}", left_str, right_str))
                        }
                        _ => unreachable!(
                            "Failed to apply {} operator on {} and {}",
                            operator.lexeme, left, right
                        ),
                    },

                    // Comparison
                    TokenType::Greater => Value::Boolean(left_num > right_num),
                    TokenType::GreaterEqual => Value::Boolean(left_num >= right_num),
                    TokenType::Less => Value::Boolean(left_num < right_num),
                    TokenType::LessEqual => Value::Boolean(left_num <= right_num),

                    // Equality
                    TokenType::BangEqual => Value::Boolean(left != right),
                    TokenType::EqualEqual => Value::Boolean(left == right),

                    _ => unreachable!(),
                }
            }
            Expression::Grouping { expression } => self.evaluate(*expression),
            Expression::Literal { value } => match value {
                Literal::String(str) => Value::String(str),
                Literal::Number(num) => Value::Number(num),
                Literal::Boolean(bool) => Value::Boolean(bool),
                Literal::None => Value::Nil,
            },
            Expression::Unary { operator, right } => {
                let right_val = self.evaluate(*right);
                match operator.token_type {
                    TokenType::Bang => Value::Boolean(!right_val.is_truthy()),
                    TokenType::Minus => {
                        if let Value::Number(num) = right_val {
                            Value::Number(-num)
                        } else {
                            panic!("Tried to apply unsupported unary operation")
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(PartialEq)]
enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(str) => str.to_string(),
                Self::Number(num) => num.to_string(),
                Self::Boolean(bool) => bool.to_string(),
                Self::Nil => "nil".to_string(),
            }
        )
    }
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::String(_) | Value::Number(_) => false,
            Value::Boolean(bool) => !bool,
            Value::Nil => false,
        }
    }
}
