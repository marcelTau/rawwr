use crate::token::{Token, TokenType};
use crate::object::*;

#[derive(Debug, Clone)]
pub enum LoxResult {
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    Error { line: usize, message: String },
    ReturnValue { value: Object },
    SystemError { message: String },
}

impl LoxResult {
    pub fn runtime_error(token: &Token, message: &str) -> LoxResult {
        let e = LoxResult::RuntimeError {
            token: token.clone(),
            message: message.to_string(),
        };
        e.report("");
        e
    }

    pub fn parse_error(token: &Token, message: &str) -> LoxResult {
        let e = LoxResult::ParseError {
            token: token.clone(),
            message: message.to_string(),
        };
        e.report("");
        e
    }

    pub fn scanner_error(line: usize, message: &str) -> LoxResult {
        let e = LoxResult::Error {
            line,
            message: message.to_string(),
        };
        e.report("");
        e
    }

    pub fn system_error(message: &str) -> LoxResult {
        let e = LoxResult::SystemError {
            message: message.to_string()
        };
        e.report("");
        e
    }

    pub fn return_value(value: Object) -> LoxResult {
        LoxResult::ReturnValue { value }
    }

    pub fn report(&self, msg: &str) {
        match self {
            LoxResult::ParseError { token, message }
            | LoxResult::RuntimeError { token, message } => {
                if token.token_type == TokenType::EOF {
                    eprintln!("[line: {}] at end {}", token.line, message);
                } else {
                    eprintln!("[line: {}] at '{}' {}", token.line, token.lexeme, message);
                }
            }
            LoxResult::Error { line, message } => {
                eprintln!("[line: {}] Error {}: {}", line, msg, message);
            }
            LoxResult::SystemError { message } => {
                eprintln!("System error: {}", message);
            }
            LoxResult::ReturnValue { .. } => ()
        }
    }
}
