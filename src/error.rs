use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct LoxError {
    token: Option<Token>,
    line: i32,
    message: String,
}

impl LoxError {
    pub fn runtime_error(token: &Token, message: &str) -> LoxError {
        let e = LoxError {
            token: Some(token.clone()),
            line: token.line,
            message: message.to_string(),
        };
        e.report("");
        e
    }

    pub fn parse_error(token: &Token, message: &str) -> LoxError {
        let e = LoxError {
            token: Some(token.clone()),
            line: token.line,
            message: message.to_string(),
        };
        e.report("");
        e
    }

    pub fn scanner_error(line: i32, message: &str) -> LoxError {
        let e = LoxError {
            token: None,
            line,
            message: message.to_string(),
        };
        e.report("");
        e
    }

    pub fn system_error(message: &str) -> LoxError {
        let e = LoxError {
            token: None,
            line: -1,
            message: message.to_string()
        };
        e.report("");
        e
    }

    pub fn report(&self, msg: &str) {
        if let Some(token) = &self.token {
            if token.token_type == TokenType::EOF {
                eprintln!("line:{} at end {}", token.line, self.message);
            } else {
                eprintln!("line:{} at '{}' {}", token.line, token, self.message);
            }
        } else {
            eprintln!("[line {}] Error {}: {}", self.line, msg, self.message);
        }
    }
}
