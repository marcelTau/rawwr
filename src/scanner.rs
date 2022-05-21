use std::collections::HashMap;

use crate::token::{TokenType, Token, Literal};
use crate::utils::{is_digit, is_alpha, is_alphanumeric};

#[derive(Debug, Clone)]
pub enum ScannerError {
    Default,
    Error {
        line: i32,
        message: String,
    },
}

impl ScannerError {
    pub fn report(error: &ScannerError) -> () {
        match error {
            ScannerError::Error {
                line,
                message,
            } => {
                println!("{} at (line: {})", message, line);
            }
            _ => (),
        }
    }
}

pub struct Scanner {
    error: ScannerError,
    source_code: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: i32,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source_code: &String) -> Self {
        let keywords = HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]);
        Scanner {
            error: ScannerError::Default,
            tokens: Vec::<Token>::new(),
            source_code: source_code.clone(),
            current: 0,
            start: 0,
            line: 1,
            keywords,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source_code.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source_code.chars().nth(self.current).unwrap();
        self.current += 1;
        return c;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = &self.source_code[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            lexeme.to_string(),
            literal.unwrap_or(Literal::new()),
            self.line
        ));
    }

    fn add_token_single(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn expect(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source_code.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source_code.chars().nth(self.current).unwrap();
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source_code.len() {
            return '\0';
        }
        self.source_code.chars().nth(self.current + 1).unwrap()
    }

    fn new_line(&mut self) {
        self.line += 1;
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.new_line();
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error = ScannerError::Error {
                line: self.line,
                message: "Unterminated String.".to_string(),
            };
            return;
        }

        self.advance();

        let literal = self.source_code[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token(
            TokenType::StringLiteral,
            Some(Literal::new_string(literal.to_string().clone())),
        );
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let literal: f64 = self.source_code[self.start..self.current]
            .to_string()
            .parse()
            .unwrap();
        self.add_token(TokenType::NumberLiteral, Some(Literal::new_number(literal)));
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source_code[self.start..self.current];
        let token_type = self
            .keywords
            .get(text)
            .unwrap_or(&TokenType::Identifier)
            .clone();
        self.add_token_single(token_type);
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token_single(TokenType::LeftParen),
            ')' => self.add_token_single(TokenType::RightParen),
            '{' => self.add_token_single(TokenType::LeftBrace),
            '}' => self.add_token_single(TokenType::RightBrace),
            ',' => self.add_token_single(TokenType::Comma),
            '.' => self.add_token_single(TokenType::Dot),
            '-' => self.add_token_single(TokenType::Minus),
            '+' => self.add_token_single(TokenType::Plus),
            ';' => self.add_token_single(TokenType::Semicolon),
            '*' => self.add_token_single(TokenType::Star),
            '!' => {
                if self.expect('=') {
                    self.add_token_single(TokenType::BangEqual)
                } else {
                    self.add_token_single(TokenType::Bang)
                }
            }
            '=' => {
                if self.expect('=') {
                    self.add_token_single(TokenType::EqualEqual)
                } else {
                    self.add_token_single(TokenType::Equal)
                }
            }
            '<' => {
                if self.expect('=') {
                    self.add_token_single(TokenType::LessEqual)
                } else {
                    self.add_token_single(TokenType::Less)
                }
            }
            '>' => {
                if self.expect('=') {
                    self.add_token_single(TokenType::GreaterEqual)
                } else {
                    self.add_token_single(TokenType::Greater)
                }
            }
            '/' => {
                if self.expect('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_single(TokenType::Slash)
                }
            }

            ' ' | '\t' | '\r' => {}

            '\n' => {
                self.new_line();
            }

            '\"' => {
                self.string();
            }

            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    self.error = ScannerError::Error {
                        line: self.line,
                        message: "Unexpected Character.".to_string(),
                    };
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ScannerError> {
        let emit_token = |token_type: TokenType| {
            let lexeme = &self.source_code[self.start..self.current];
        };

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();

            match &self.error {
                ScannerError::Error { .. } => {
                    return Err(self.error.clone());
                }
                _ => ()
            }
        }

        self.add_token_single(TokenType::EOF);
        Ok(self.tokens.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_assignment() {
        let code = "var x = 10;\n".to_string();
        let mut scanner = Scanner::new(&code);

        let expected: Vec<Token> = [
            Token::new(TokenType::Var, "var".to_string(), Literal::new(), 1),
            Token::new(TokenType::Identifier, "x".to_string(), Literal::new(), 1),
            Token::new(TokenType::Equal, "=".to_string(), Literal::new(), 1),
            Token::new(TokenType::NumberLiteral, "10".to_string(), Literal::new_number(10 as f64), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), Literal::new(), 1),
            Token::new(TokenType::EOF, "\n".to_string(), Literal::new(), 2),
        ].into_iter().collect();

        let tokens = scanner.tokenize().unwrap();
        tokens.iter().zip(&expected).for_each(|(a, b)| assert_eq!(a, b));
    }

    #[test]
    fn string_assignment() {
        let code = "var x = \"hallo\";\n".to_string();
        let mut scanner = Scanner::new(&code);

        let expected: Vec<Token> = [
            Token::new(TokenType::Var, "var".to_string(), Literal::new(), 1),
            Token::new(TokenType::Identifier, "x".to_string(), Literal::new(), 1),
            Token::new(TokenType::Equal, "=".to_string(), Literal::new(), 1),
            Token::new(TokenType::StringLiteral, "\"hallo\"".to_string(), Literal::new_string("hallo".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), Literal::new(), 1),
            Token::new(TokenType::EOF, "\n".to_string(), Literal::new(), 2),
        ].into_iter().collect();

        let tokens = scanner.tokenize().unwrap();
        tokens.iter().zip(&expected).for_each(|(a, b)| assert_eq!(a, b));
    }
}


















