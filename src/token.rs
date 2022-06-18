use crate::object::*;
use core::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLiteral,
    NumberLiteral,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Object>, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.token_type {
            TokenType::StringLiteral
            | TokenType::NumberLiteral
            | TokenType::Nil
            | TokenType::True
            | TokenType::False => {
                write!(
                    f,
                    "Found {:?} (\"{}\") \"{}\" at line {:?}",
                    self.token_type,
                    self.lexeme,
                    if let Some(literal) = &self.literal {
                        literal.to_string()
                    } else {
                        "None".to_string()
                    },
                    self.line
                )
            }
            _ => {
                write!(
                    f,
                    "Found {:?} (\"{}\") at {:?}",
                    self.token_type, self.lexeme, self.line
                )
            }
        }
    }
}
