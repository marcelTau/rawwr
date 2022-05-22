use crate::expr::*;
use crate::scanner::*;
use crate::token::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.clone(),
            current: 0,
        }
    }

    fn is_match(&mut self, token_types: &Vec<TokenType>) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn peek(&self) -> Token {
        self.tokens[self.current + 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn error(&self, error: &ScannerError) {
        ScannerError::report(error);
    }

    fn consume(&mut self, token_type: &TokenType, message: String) -> Result<Token, ScannerError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let err = ScannerError::Error {
                line: self.peek().line,
                message
            };
            self.error(&err);
            Err(err)
        }
    }

    // ------------------------------------------------------------------------

    fn expression(&mut self) -> Result<Expr, ScannerError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ScannerError> {
        let mut expr = self.comparison()?;
        while self.is_match(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ScannerError> {
        let mut expr = self.term()?;
        while self.is_match(&vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ScannerError> {
        let mut expr = self.factor()?;

        while self.is_match(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ScannerError> {
        let mut expr = self.unary()?;

        while self.is_match(&vec![TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ScannerError> {
        if self.is_match(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ScannerError> {
        match self.tokens[self.current].token_type {
            TokenType::False => Ok(Expr::Literal(LiteralExpr {
                value: Literal::new_bool(false),
            })),
            TokenType::True => Ok(Expr::Literal(LiteralExpr {
                value: Literal::new_bool(true),
            })),
            TokenType::Nil => Ok(Expr::Literal(LiteralExpr {
                value: Literal::new_nil(),
            })),
            TokenType::StringLiteral => Ok(Expr::Literal(LiteralExpr {
                value: Literal::new_string(self.previous().literal.as_string()),
            })),
            TokenType::NumberLiteral => Ok(Expr::Literal(LiteralExpr {
                value: Literal::new_number(self.previous().literal.as_string().parse().unwrap()),
            })),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(&TokenType::RightParen, "Expect ')' after expression.".to_string())?;
                Ok(Expr::Grouping(GroupingExpr {
                    expression: Box::new(expr),
                }))
            }
            _ => unreachable!(),
        }
    }
}
