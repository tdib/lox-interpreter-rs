use std::fmt::Display;

use crate::error::{runtime_error, RuntimeError, RuntimeResult};
use crate::expression::Expression;
use crate::token::{Literal, Token, TokenType};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&self, expression: Expression) {
        let value = Self::evaluate(expression);
        match value {
            Ok(value) => println!("{}", value),
            Err(error) => runtime_error(error),
        }
    }

    fn evaluate(expression: Expression) -> RuntimeResult<Value> {
        match expression {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = Self::evaluate(*left)?;
                let right = Self::evaluate(*right)?;

                match operator.token_type {
                    // Arithmetic
                    TokenType::Minus => {
                        let (l_num, r_num) = Self::check_number_operands(operator, left, right)?;
                        Ok(Value::Number(l_num - r_num))
                    }
                    TokenType::Slash => {
                        let (l_num, r_num) = Self::check_number_operands(operator, left, right)?;
                        Ok(Value::Number(l_num / r_num))
                    }
                    TokenType::Star => {
                        let (l_num, r_num) = Self::check_number_operands(operator, left, right)?;
                        Ok(Value::Number(l_num * r_num))
                    }
                    TokenType::Plus => match (&left, &right) {
                        (Value::Number(left_num), Value::Number(right_num)) => {
                            Ok(Value::Number(left_num + right_num))
                        }
                        (Value::String(left_str), Value::String(right_str)) => {
                            Ok(Value::String(format!("{}{}", left_str, right_str)))
                        }
                        _ => Err(RuntimeError::new(
                            format!(
                                "Operands '{}' and '{}' must both be numbers or strings.",
                                left, right,
                            ),
                            operator,
                        )),
                    },

                    // Comparison
                    TokenType::Greater => {
                        let (l_num, r_num) = Self::check_number_operands(operator, left, right)?;
                        Ok(Value::Boolean(l_num > r_num))
                    }
                    TokenType::GreaterEqual => {
                        let (l_num, r_num) = Self::check_number_operands(operator, left, right)?;
                        Ok(Value::Boolean(l_num >= r_num))
                    }
                    TokenType::Less => {
                        let (l_num, r_num) = Self::check_number_operands(operator, left, right)?;
                        Ok(Value::Boolean(l_num < r_num))
                    }
                    TokenType::LessEqual => {
                        let (l_num, r_num) = Self::check_number_operands(operator, left, right)?;
                        Ok(Value::Boolean(l_num <= r_num))
                    }

                    // Equality
                    TokenType::BangEqual => Ok(Value::Boolean(left != right)),
                    TokenType::EqualEqual => Ok(Value::Boolean(left == right)),

                    _ => unreachable!(
                        "Operator '{}' was not handled as a binary expression",
                        operator
                    ),
                }
            }
            Expression::Grouping { expression } => Self::evaluate(*expression),
            Expression::Literal { value } => match value {
                Literal::String(str) => Ok(Value::String(str)),
                Literal::Number(num) => Ok(Value::Number(num)),
                Literal::Boolean(bool) => Ok(Value::Boolean(bool)),
                Literal::None => Ok(Value::Nil),
            },
            Expression::Unary { operator, right } => {
                let right_val = Self::evaluate(*right)?;
                match operator.token_type {
                    TokenType::Bang => Ok(Value::Boolean(!right_val.is_truthy())),
                    TokenType::Minus => {
                        if let Value::Number(num) = right_val {
                            Ok(Value::Number(-num))
                        } else {
                            Err(RuntimeError::new(
                                format!(
                                    "Operand '{}' must be a number to apply '{}' operator",
                                    right_val, operator
                                ),
                                operator,
                            ))
                        }
                    }
                    _ => unreachable!(
                        "Operator '{}' was not handled as a unary expression",
                        operator
                    ),
                }
            }
        }
    }

    fn check_number_operands(
        operator: Token,
        left: Value,
        right: Value,
    ) -> RuntimeResult<(f64, f64)> {
        match (&left, &right) {
            (Value::Number(left_num), Value::Number(right_num)) => Ok((*left_num, *right_num)),
            _ => Err(RuntimeError::new(
                format!("Operands '{}' and '{}' must both be numbers.", left, right),
                operator,
            )),
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

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::String(_) | Value::Number(_) => false,
            Value::Boolean(bool) => !bool,
            Value::Nil => false,
        }
    }
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
