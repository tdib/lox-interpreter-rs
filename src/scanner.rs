use std::collections::HashMap;

use crate::error::lox_generic_error;
use crate::token::{Literal, Token, TokenType};
use crate::util::GenericScanner;

use lazy_static::lazy_static;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Literal::None,
            self.line,
        ));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.consume();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.check_and_consume(&['=']) {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.check_and_consume(&['=']) {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.check_and_consume(&['=']) {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.check_and_consume(&['=']) {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.check_and_consume(&['/']) {
                    // We have encountered a comment so we will scan until we reach the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.consume();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            // Ignore whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            // String
            '"' => self.parse_string(),

            // Number
            c if c.is_ascii_digit() => self.parse_number(),

            // Identifier (variable name/keywords)
            c if Self::is_valid_identifier_char(c) => self.parse_identifier(),

            // TODO: Dot?
            '.' => self.add_token(TokenType::Dot),

            _ => lox_generic_error(self.line, &format!("Unexpected character '{c}'")),
        };
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_value(token_type, Literal::None);
    }

    fn add_token_with_value(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn parse_string(&mut self) {
        // Consume until we reach the end of the string or the input
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.consume();
        }

        // If we hit this, it means we have an unclosed quote
        if self.is_at_end() {
            lox_generic_error(self.line, "Unterminated string.");
        }

        // Consume closing quote
        self.consume();

        // Trim the quotes off
        let string = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_value(TokenType::String, Literal::String(string));
    }

    fn parse_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.consume();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the period
            self.consume();

            while self.peek().is_ascii_digit() {
                self.consume();
            }
        }

        let number_slice = self.source[self.start..self.current].to_string();
        let number = number_slice
            .parse::<f64>()
            .unwrap_or_else(|e| panic!("Failed to parse {} as a number: {}", number_slice, e));
        self.add_token_with_value(TokenType::Number, Literal::Number(number));
    }

    fn parse_identifier(&mut self) {
        while Scanner::is_valid_identifier_char(self.peek()) {
            self.consume();
        }

        let identifier = self.source[self.start..self.current].to_string();
        let identifier_token_type = KEYWORDS.get(&identifier).unwrap_or(&TokenType::Identifier);

        match identifier_token_type {
            TokenType::True => self.add_token_with_value(TokenType::True, Literal::Boolean(true)),
            TokenType::False => {
                self.add_token_with_value(TokenType::False, Literal::Boolean(false))
            }
            _ => self.add_token(*identifier_token_type),
        }
    }

    fn get_current_char(&self) -> char {
        self.get_nth_char(self.current)
    }

    fn get_nth_char(&self, n: usize) -> char {
        self.source
            .chars()
            .nth(n)
            .unwrap_or_else(|| panic!("Failed to get character at index {}", self.current))
    }

    fn is_valid_identifier_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}

impl GenericScanner<char, char> for Scanner {
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn consume(&mut self) -> char {
        let curr_char = self.get_current_char();
        self.current += 1;
        curr_char
    }

    fn check_and_consume(&mut self, expected: &[char]) -> bool {
        if self.is_at_end()
            || expected
                .iter()
                .any(|expected| self.get_current_char() != *expected)
        {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.get_current_char()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.get_nth_char(self.current + 1)
        }
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and".to_string(), TokenType::And);
        map.insert("class".to_string(), TokenType::Class);
        map.insert("else".to_string(), TokenType::Else);
        map.insert("false".to_string(), TokenType::False);
        map.insert("for".to_string(), TokenType::For);
        map.insert("fun".to_string(), TokenType::Fun);
        map.insert("if".to_string(), TokenType::If);
        map.insert("nil".to_string(), TokenType::Nil);
        map.insert("or".to_string(), TokenType::Or);
        map.insert("print".to_string(), TokenType::Print);
        map.insert("return".to_string(), TokenType::Return);
        map.insert("super".to_string(), TokenType::Super);
        map.insert("this".to_string(), TokenType::This);
        map.insert("true".to_string(), TokenType::True);
        map.insert("var".to_string(), TokenType::Var);
        map.insert("while".to_string(), TokenType::While);
        map
    };
}
