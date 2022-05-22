use crate::expr::*;
use crate::error::*;
use crate::token::*;
use crate::object::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    error: ScannerError,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.clone(),
            current: 0,
            error: ScannerError::Default,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
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
        &self.peek().token_type == token_type
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
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
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn error(&mut self, error: &ScannerError) -> ScannerError {
        self.error = error.clone();
        // @todo this should be done in the synchronize method
        //       don't forget to reset it
        ScannerError::report(error);
        error.clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            if matches!(
                self.peek().token_type,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
            self.advance();
        }
    }

    fn consume(&mut self, token_type: &TokenType, message: String) -> Result<Token, ScannerError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let err = ScannerError::Error {
                line: self.peek().line,
                message,
            };
            Err(self.error(&err))
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
        if self.is_match(&vec![TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::False),
            }));
        }
        if self.is_match(&vec![TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::True),
            }));
        }
        if self.is_match(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }
        if self.is_match(&vec![TokenType::StringLiteral, TokenType::NumberLiteral]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        }
        if self.is_match(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                &TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            )?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }
        return Err(self.error(&ScannerError::Error {
            line: self.tokens[self.current].line,
            message: "Expect expression.".to_string(),
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_printer::AstPrinter;
    use crate::scanner::*;

    fn run(code: &String) -> String {
        let mut scanner = Scanner::new(code);
        let tokens = match scanner.tokenize() {
            Ok(tokens) => tokens,
            Err(err) => {
                ScannerError::report(&err);
                unreachable!();
            }
        };

        for t in &tokens {
            println!("token: {}", t);
        }

        let mut parser = Parser::new(tokens);
        let printer = AstPrinter {};

        match parser.parse() {
            None => "".to_string(),
            Some(expr) => printer.print(&expr).unwrap(),
        }
    }

    #[test]
    fn simple_addition() {
        let code = "4 + 5".to_string();
        let expected = "(+ 4 5)".to_string();
        assert_eq!(expected, run(&code));
    }

    #[test]
    fn operator_precedence() {
        let code = "4 + 5 * 7".to_string();
        let expected = "(+ 4 (* 5 7))".to_string();
        assert_eq!(expected, run(&code));
    }

    #[test]
    fn unary() {
        let code = "-4 + 5 * 7".to_string();
        let expected = "(+ (- 4) (* 5 7))".to_string();
        assert_eq!(expected, run(&code));
    }

    #[test]
    fn multiple_unary() {
        let code = "--4 + 5 * 7".to_string();
        let expected = "(+ (- (- 4)) (* 5 7))".to_string();
        assert_eq!(expected, run(&code));
    }

    #[test]
    fn parens_over_mulitplication_precedence() {
        let code = "4 * (1 + 2)".to_string();
        let expected = "(* 4 (group (+ 1 2)))".to_string();
        assert_eq!(expected, run(&code));
    }
}
