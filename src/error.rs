use crate::token::{Token, TokenType};
use std::sync::atomic::{AtomicBool, Ordering};

static ERROR_FLAG: AtomicBool = AtomicBool::new(false);
static RUNTIME_ERROR_FLAG: AtomicBool = AtomicBool::new(false);

pub fn lox_generic_error(line: usize, message: &str) {
    report_error(line, None, message);
}

pub fn report_error(line: usize, r#where: Option<&str>, message: &str) {
    if r#where.is_none() {
        eprintln!("[line: {}] Error: {}", line, message);
    } else {
        eprintln!(
            "[line: {}] Error {}: {}",
            line,
            r#where.expect("Error location not provided"),
            message
        );
    }
    set_error_flag(true);
}

pub fn parse_error(token: Token, message: String) {
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

pub fn runtime_error(error: RuntimeError) {
    lox_generic_error(error.token.line, &error.message);
    set_runtime_error_flag(true);
}

pub fn set_error_flag(value: bool) {
    ERROR_FLAG.store(value, Ordering::SeqCst);
}

pub fn get_error_flag() -> bool {
    ERROR_FLAG.load(Ordering::SeqCst)
}

fn set_runtime_error_flag(value: bool) {
    RUNTIME_ERROR_FLAG.store(value, Ordering::SeqCst);
}

pub fn get_runtime_error_flag() -> bool {
    RUNTIME_ERROR_FLAG.load(Ordering::SeqCst)
}

pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: String) -> Self {
        ParseError { message }
    }
}
pub type ParseResult<T> = std::result::Result<T, ParseError>;

pub struct RuntimeError {
    pub message: String,
    token: Token,
}

impl RuntimeError {
    pub fn new(message: String, token: Token) -> Self {
        RuntimeError { message, token }
    }
}
pub type RuntimeResult<T> = std::result::Result<T, RuntimeError>;
