use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
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
pub struct IsNil(bool);

#[derive(Debug, Clone, PartialEq)]
pub struct Literal(Option<f64>, Option<String>, Option<bool>, Option<IsNil>);

impl Literal {
    pub fn new() -> Self {
        Literal(None, None, None, None)
    }

    pub fn new_number(number: f64) -> Self {
        Literal(Some(number), None, None, None)
    }

    pub fn new_string(string: String) -> Self {
        Literal(None, Some(string), None, None)
    }

    pub fn new_bool(boolean: bool) -> Self {
        Literal(None, None, Some(boolean), None)
    }

    pub fn new_nil() -> Self {
        Literal(None, None, None, Some(IsNil(true)))
    }

    pub fn as_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output: String = match self {
            Literal(Some(x), ..) => x.to_string(),
            Literal(None, Some(x), ..) => x.to_string(),
            Literal(None, None, Some(x), ..) => x.to_string(),
            //Literal(None, None, None, Some(x)) => x.0.to_string(), //@todo is this actually "nil"
            Literal(None, None, None, Some(x)) => if x.0 { "nil".to_string() } else { "".to_string() },
            _ => "unimplemented".to_string(),
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    pub lexeme: String,
    literal: Literal,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: i32) -> Self {
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
                    "Found {:?} (\"{}\") \"{}\" at {:?}",
                    self.token_type, self.lexeme, self.literal, self.line
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
