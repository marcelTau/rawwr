use crate::error::*;
use crate::expr::*;
use crate::object::*;
use crate::stmt::*;
use crate::token::*;

use std::rc::Rc;

use crate::token::TokenType::*;

macro_rules! match_token {
    ($self:ident, $($args:ident),+) => {
        $self.is_match(&[$(TokenType::$args),*])
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, LoxResult> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn is_match(&mut self, token_types: &[TokenType]) -> bool {
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
        self.peek().token_type == EOF
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            if matches!(
                self.peek().token_type,
                Class | Fun | Var | For | If | While | Print | Return
            ) {
                return;
            }
            self.advance();
        }
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Token, LoxResult> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(&self.peek(), message))
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> LoxResult {
        self.had_error = true;
        LoxResult::parse_error(token, message)
    }

    pub fn success(&self) -> bool {
        !self.had_error
    }

    // ------------------------------------------------------------------------

    fn expression(&mut self) -> Result<Expr, LoxResult> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        let result = if match_token!(self, Class) {
            self.class_declaration()
        } else if match_token!(self, Fun) {
            self.function("function")
        } else if match_token!(self, Var) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    fn class_declaration(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        let name = self.consume(&Identifier, "Expect class name.")?;
        self.consume(&LeftBrace, "Expect '{{' before class body.")?;

        let mut methods = Vec::new();

        while !self.check(&RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }

        self.consume(&RightBrace, "Expect '}}' after class body.")?;

        Ok(Rc::new(Stmt::Class(Rc::new(ClassStmt {
            name,
            methods: Rc::new(methods),
        }))))
    }

    fn statement(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        if match_token!(self, For) {
            return self.for_statement();
        }
        if match_token!(self, If) {
            return Ok(Rc::new(self.if_statement()?));
        }

        if match_token!(self, Print) {
            return Ok(Rc::new(self.print_statement()?));
        }

        if match_token!(self, Return) {
            return Ok(Rc::new(self.return_statement()?));
        }

        if match_token!(self, While) {
            return Ok(Rc::new(self.while_statement()?));
        }

        if match_token!(self, LeftBrace) {
            return Ok(Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(self.block()?),
            }))));
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        self.consume(&LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if match_token!(self, Semicolon) {
            None
        } else if match_token!(self, Var) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&Semicolon) {
            self.expression()?
        } else {
            Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::Bool(true)),
            }))
        };

        self.consume(&Semicolon, "Expect ';' after loop condition")?;

        let increment = if !self.check(&RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(&RightParen, "Expect ')' after for clauses")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(vec![
                    body,
                    Rc::new(Stmt::Expression(Rc::new(ExpressionStmt {
                        expression: Rc::new(inc),
                    }))),
                ]),
            })));
        };

        body = Rc::new(Stmt::While(Rc::new(WhileStmt {
            condition: Rc::new(condition),
            body,
        })));

        if let Some(init) = initializer {
            body = Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(vec![init, body]),
            })));
        };

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(&LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if match_token!(self, Else) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Stmt::If(Rc::new(IfStmt {
            condition: Rc::new(condition),
            then_branch,
            else_branch,
        })))
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxResult> {
        let value = self.expression()?;
        self.consume(&Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Rc::new(PrintStmt {
            expression: Rc::new(value),
        })))
    }

    fn return_statement(&mut self) -> Result<Stmt, LoxResult> {
        let keyword = self.previous();

        let value = if !self.check(&Semicolon) {
            Some(Rc::new(self.expression()?))
        } else {
            None
        };

        self.consume(&Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(Rc::new(ReturnStmt { keyword, value })))
    }

    fn var_declaration(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        let name = self.consume(&Identifier, "Expect variable name.")?;

        let initializer = if match_token!(self, Equal) {
            Some(Rc::new(self.expression()?))
        } else {
            None
        };

        self.consume(&Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Rc::new(Stmt::Var(Rc::new(VarStmt { name, initializer }))))
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(&LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&RightParen, "Expect ')' after condition.")?;

        let body = self.statement()?;

        Ok(Stmt::While(Rc::new(WhileStmt {
            condition: Rc::new(condition),
            body,
        })))
    }

    fn expression_statement(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        let expr = self.expression()?;
        self.consume(&Semicolon, "Expect ';' after expression.")?;
        Ok(Rc::new(Stmt::Expression(Rc::new(ExpressionStmt {
            expression: Rc::new(expr),
        }))))
    }

    fn function(&mut self, kind: &str) -> Result<Rc<Stmt>, LoxResult> {
        let name = self.consume(&Identifier, &format!("Expect {kind} name."))?;
        self.consume(&LeftParen, &format!("Expect '(' after {kind} name."))?;

        let mut params = Vec::new();

        if !self.check(&RightParen) {
            params.push(self.consume(&Identifier, "Expect parameter name.")?);
            while match_token!(self, Comma) {
                if params.len() >= 255 {
                    self.error(&self.peek(), "You can't have more than 255 parameters.");
                }
                params.push(self.consume(&Identifier, "Expect parameter name.")?);
            }
        }
        self.consume(&RightParen, "Expect ')' after parameters.")?;
        self.consume(&LeftBrace, &format!("Expect '{{' before {kind} body."))?;

        let body = self.block()?;
        Ok(Rc::new(Stmt::Function(Rc::new(FunctionStmt {
            name,
            params: Rc::new(params),
            body: Rc::new(body),
        }))))
    }

    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, LoxResult> {
        let mut statements = vec![];

        while !self.check(&RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(&RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, LoxResult> {
        let expr = self.or()?;

        if match_token!(self, Equal) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(expr) = expr {
                return Ok(Expr::Assign(Rc::new(AssignExpr {
                    name: expr.name.clone(),
                    value: Rc::new(value),
                })));
            } else if let Expr::Get(expr) = expr {
                return Ok(Expr::Set(Rc::new(SetExpr {
                    object: Rc::clone(&expr.object),
                    name: expr.name.clone(),
                    value: Rc::new(value),
                })));
            };
            self.error(&equals, "Invalid assignment target");
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.and()?;

        while match_token!(self, Or) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Rc::new(LogicalExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.equality()?;

        while match_token!(self, And) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Rc::new(LogicalExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.comparison()?;

        while match_token!(self, BangEqual, EqualEqual) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.term()?;

        while match_token!(self, Greater, GreaterEqual, Less, LessEqual) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.factor()?;

        while match_token!(self, Minus, Plus) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }))
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.unary()?;

        while match_token!(self, Star, Slash) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }))
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxResult> {
        if match_token!(self, Bang, Minus) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(Rc::new(UnaryExpr {
                operator,
                right: Rc::new(right),
            })))
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.primary()?;

        loop {
            if match_token!(self, LeftParen) {
                expr = self.finish_call(&Rc::new(expr))?;
            } else if match_token!(self, Dot) {
                let name = self.consume(&Identifier, "Expect property name after '.'.")?;
                expr = Expr::Get(Rc::new(GetExpr {
                    object: Rc::new(expr),
                    name,
                }))
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: &Rc<Expr>) -> Result<Expr, LoxResult> {
        let mut arguments = Vec::new();

        if !self.check(&RightParen) {
            arguments.push(Rc::new(self.expression()?));
            while match_token!(self, Comma) {
                if arguments.len() >= 255 {
                    self.error(&self.peek(), "You can't have more than 255 arguments.");
                }
                arguments.push(Rc::new(self.expression()?));
            }
        }

        let paren = self.consume(&RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call(Rc::new(CallExpr {
            callee: Rc::clone(callee),
            paren,
            arguments,
        })))
    }

    fn primary(&mut self) -> Result<Expr, LoxResult> {
        if match_token!(self, False) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::Bool(false)),
            })));
        }
        if match_token!(self, True) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::Bool(true)),
            })));
        }
        if match_token!(self, Nil) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::Nil),
            })));
        }
        if match_token!(self, StringLiteral, NumberLiteral) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: self.previous().literal,
            })));
        }

        if match_token!(self, This) {
            return Ok(Expr::This(Rc::new(ThisExpr {
                keyword: self.previous(),
            })));
        }

        if match_token!(self, Identifier) {
            return Ok(Expr::Variable(Rc::new(VariableExpr {
                name: self.previous(),
            })));
        }

        if match_token!(self, LeftParen) {
            let expr = self.expression()?;
            self.consume(&RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Rc::new(GroupingExpr {
                expression: Rc::new(expr),
            })));
        }

        Err(LoxResult::parse_error(
            &self.tokens[self.current],
            "Expect Expression",
        ))
    }
}
/*
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
                //LoxError::report(&err); //@todo recheck
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
*/
