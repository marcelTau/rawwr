use crate::token::{Token, TokenType};

//#[derive(Debug, Clone)]
//pub struct LoxResult {
    //token: Option<Token>,
    //line: i32,
    //message: String,
//}

#[derive(Debug, Clone)]
pub enum LoxResult {
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    Error { line: usize, message: String },
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
        }
    }
}
