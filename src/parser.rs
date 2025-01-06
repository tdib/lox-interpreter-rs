use crate::expression::Expression;
use crate::report_error;
use crate::token::{Literal, Token, TokenType};
use crate::util::GenericScanner;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse_error(token: Token, message: String) {
        if token.token_type == TokenType::Eof {
            report_error(token.line, Some("at end of input"), &message)
        } else {
            report_error(
                token.line,
                Some(&format!("at '{}'", token.lexeme)),
                &message,
            )
        }
    }

    pub fn parse(&mut self) -> Option<Expression> {
        match self.parse_expression() {
            Ok(expression) => Some(expression),

            Err(parse_error) => {
                Parser::parse_error(self.peek(), parse_error.message);
                self.synchronise();
                None
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expression> {
        let mut expression = self.parse_comparison()?;

        while self.check_and_consume(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.peek_previous();
            let right = self.parse_comparison()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut expression = self.parse_term()?;

        while self.check_and_consume(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.peek_previous();
            let right = self.parse_term()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<Expression> {
        let mut expression = self.parse_factor()?;

        while self.check_and_consume(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.peek_previous();
            let right = self.parse_factor()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<Expression> {
        let mut expression = self.parse_unary()?;

        while self.check_and_consume(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.peek_previous();
            let right = self.parse_unary()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        if self.check_and_consume(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.peek_previous();
            let right = self.parse_unary()?;
            Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.parse_literal_or_group()
        }
    }

    fn parse_literal_or_group(&mut self) -> Result<Expression> {
        let curr_literal = self.peek().literal;

        let match_result = match self.peek().token_type {
            TokenType::False | TokenType::True => {
                if let Literal::Boolean(bool) = curr_literal {
                    Ok(Expression::Literal {
                        value: Literal::Boolean(bool),
                    })
                } else {
                    Err(ParseError::new(format!(
                        "Failed to convert literal {:?} to boolean.",
                        curr_literal
                    )))
                }
            }

            TokenType::Nil => Ok(Expression::Literal {
                value: Literal::None,
            }),

            TokenType::Number => {
                if let Literal::Number(num) = curr_literal {
                    Ok(Expression::Literal {
                        value: Literal::Number(num),
                    })
                } else {
                    Err(ParseError::new(format!(
                        "Failed to convert literal {:?} to number.",
                        curr_literal
                    )))
                }
            }

            TokenType::String => {
                if let Literal::String(str) = curr_literal {
                    Ok(Expression::Literal {
                        value: Literal::String(str),
                    })
                } else {
                    Err(ParseError::new(format!(
                        "Failed to convert literal {:?} to string.",
                        curr_literal
                    )))
                }
            }

            TokenType::LeftParen => {
                let expression = self.parse_expression()?;
                if self.check_and_consume(&[TokenType::RightParen]) {
                    Ok(Expression::Grouping {
                        expression: Box::new(expression),
                    })
                } else {
                    Err(ParseError::new(
                        "Expected ')' after expression.".to_string(),
                    ))
                }
            }

            _ => Err(ParseError::new(format!(
                "Token {} parsing was unhandled.",
                self.peek()
            ))),
        };

        if match_result.is_ok() {
            self.consume();
        }

        match_result
    }

    fn peek_previous(&self) -> Token {
        println!("current: {}", self.current);
        self.tokens
            .get(self.current - 1)
            .unwrap_or_else(|| panic!("Failed to get token at index {}", self.current))
            .clone()
    }

    /// Given some invalid syntax, discard the invalid parts until we are left with only valid
    /// syntax so we can continue parsing and check other parts of the code.
    fn synchronise(&mut self) {
        self.consume();

        while !self.is_at_end() {
            if self.peek_previous().token_type == TokenType::Semicolon {
                return;
            }

            // If we hit a token of one of these types, we can essentially "restart" parsing as if
            // we did not encounter an error
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.consume();
                }
            }
        }
    }
}

struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> Self {
        ParseError { message }
    }
}
type Result<T> = std::result::Result<T, ParseError>;

impl GenericScanner<Token, TokenType> for Parser {
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn consume(&mut self) -> Token {
        let token = self.peek();
        self.current += 1;
        token
    }

    fn check_and_consume(&mut self, expected: &[TokenType]) -> bool {
        if expected
            .iter()
            .any(|expected_token_type| self.peek().token_type == *expected_token_type)
        {
            self.consume();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| panic!("Failed to get token at index {}", self.current))
            .clone()
    }

    fn peek_next(&self) -> Token {
        self.tokens
            .get(self.current + 1)
            .unwrap_or_else(|| panic!("Failed to get token at index {}", self.current + 1))
            .clone()
    }
}
