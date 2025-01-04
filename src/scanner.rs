use std::collections::HashMap;

use crate::lox_error;
use crate::token::{Literal, Token, TokenType};

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
            Literal::Nil,
            self.line,
        ));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance_single_char();
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
                let token_type = if self.check_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.check_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.check_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.check_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.check_next('/') {
                    // We have encountered a comment so we will scan until we reach the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance_single_char();
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

            _ => lox_error(self.line, &format!("Unexpected character '{c}'")),
        };
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_value(token_type, Literal::Nil);
    }

    // TODO: Rename this to something better
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
            self.advance_single_char();
        }

        // If we hit this, it means we have an unclosed quote
        if self.is_at_end() {
            lox_error(self.line, "Unterminated string.");
        }

        // Consume closing quote
        self.advance_single_char();

        // Trim the quotes off
        let string = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_value(TokenType::String, Literal::String(string));
    }

    fn parse_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance_single_char();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the period
            self.advance_single_char();

            while self.peek().is_ascii_digit() {
                self.advance_single_char();
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
            self.advance_single_char();
        }

        let identifier = self.source[self.start..self.current].to_string();
        let identifier_token_type = KEYWORDS.get(&identifier).unwrap_or(&TokenType::Identifier);
        self.add_token(*identifier_token_type);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn check_next(&mut self, expected_next: char) -> bool {
        if self.is_at_end() || self.get_current_char() != expected_next {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance_single_char(&mut self) -> char {
        let curr_char = self.get_current_char();
        self.current += 1;
        curr_char
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
