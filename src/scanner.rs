enum TokenType {
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
    Slansh,
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
    Number,

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

#[derive(Debug)]
pub enum ScannerError {
    Default,
    Error {
        line: i32,
        column: i32,
        message: String,
    },
}

impl ScannerError {
    pub fn report(error: ScannerError) -> () {
        match error {
            ScannerError::Error {
                line,
                column,
                message,
            } => {
                println!("{} at ({}:{})", message, line, column);
            }
            _ => (),
        }
    }
}

pub struct Scanner {
    error: ScannerError,
    source_code: String,
}

impl Scanner {
    pub fn new(source_code: &String) -> Self {
        Scanner {
            error: ScannerError::Default,
            source_code: source_code.clone(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ScannerError> {
        Ok(vec![])
    }
}
