use core::fmt;

pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, Clone)]
struct Literal(Option<f64>, Option<String>);

impl Literal {
    fn new() -> Self {
        Literal(None, None)
    }

    fn new_number(number: f64) -> Self {
        Literal(Some(number), None)
    }

    fn new_string(string: String) -> Self {
        Literal(None, Some(string))
    }
}

#[derive(Debug, Clone)]
struct Position {
    line: i32,
    column: i32,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    position: Position,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Literal, position: Position) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            position,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?} at {:?}",
            self.token_type, self.lexeme, self.literal, self.position
        )
    }
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
    tokens: Vec<Token>,
    index: usize,
    start: usize,
    line: i32,
    column: i32,
}

impl Scanner {
    pub fn new(source_code: &String) -> Self {
        Scanner {
            error: ScannerError::Default,
            tokens: Vec::<Token>::new(),
            source_code: source_code.clone(),
            index: 0,
            start: 0,
            line: 0,
            column: 0,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.index >= self.source_code.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source_code.chars().nth(self.index).unwrap();
        self.index += 1;
        self.column += 1;
        return c;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = &self.source_code[self.start..self.index];
        self.tokens.push(Token::new(
            token_type,
            lexeme.to_string(),
            literal.unwrap_or(Literal::new()),
            Position {
                line: self.line,
                column: self.column,
            },
        ));
    }

    fn add_token_single(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn expect(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source_code.chars().nth(self.index).unwrap() != expected {
            return false;
        }

        self.index += 1;
        self.column += 1;

        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source_code.chars().nth(self.index).unwrap();
    }

    fn peek_next(&mut self) -> char {
        if self.index + 1 >= self.source_code.len() {
            return '\0';
        }
        self.source_code.chars().nth(self.index + 1).unwrap()
    }

    fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
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
                column: self.column,
                message: "Unterminated String.".to_string(),
            };
            return;
        }

        self.advance();

        let literal: String = self.source_code[self.start + 1..self.index - 1].to_string();
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
        let literal: f64 = self.source_code[self.start + 1..self.index - 1].to_string().parse().unwrap();
        self.add_token(
            TokenType::StringLiteral,
            Some(Literal::new_number(literal)),
        );
    }


    pub fn tokenize(&mut self) -> Result<Vec<Token>, ScannerError> {
        let emit_token = |token_type: TokenType| {
            let lexeme = &self.source_code[self.start..self.index];
        };

        while !self.is_at_end() {
            self.start = self.index;
            match self.advance() {
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
                    if is_digit(self.source_code.chars().nth(self.index).unwrap()) {
                        self.number();
                        break;
                    }
                    
                    self.error = ScannerError::Error {
                        line: self.line,
                        column: self.column,
                        message: "Unexpected Character.".to_string(),
                    };
                    break;
                }
            }
        }
        Ok(self.tokens.clone())
    }
}
