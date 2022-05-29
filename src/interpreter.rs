use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::*;
use crate::error::LoxError;
use crate::expr::*;
use crate::object::Object;
use crate::stmt::*;
use crate::token::TokenType;

pub struct Interpreter {
    environment: RefCell<Rc<RefCell<Environment>>>,
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxError> {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(initializer)?
        } else {
            Object::Nil
        };
        self.environment
            .borrow()
            .borrow_mut()
            .define(&stmt.name.lexeme, value);
        Ok(())
    }

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxError> {
        let e = Environment::new_with_enclosing(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, e)
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxError> {
        if self.is_truthy(&self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        let result = match expr.operator.token_type {
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Greater => Object::Bool(left > right),
            TokenType::GreaterEqual => Object::Bool(left >= right),
            TokenType::Less => Object::Bool(left < right),
            TokenType::LessEqual => Object::Bool(left <= right),
            TokenType::BangEqual => Object::Bool(left != right),
            TokenType::EqualEqual => Object::Bool(left == right),
            _ => unreachable!(),
        };

        match result {
            Object::ArithmeticError | Object::DivByZeroError => Err(LoxError::runtime_error(
                &expr.operator,
                format!("{}", result).as_str(),
            )),
            _ => Ok(result),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Object::Num(n) => Ok(Object::Num(n * (-1_f64))),
                _ => Ok(Object::Nil),
            },
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
            _ => Err(LoxError::runtime_error(&expr.operator, "Unreachable")),
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, LoxError> {
        self.environment.borrow().borrow().get(&expr.name)
    }

    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow()
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(&expr.right)
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: RefCell::new(Rc::new(RefCell::new(Environment::new()))),
        }
    }

    pub fn interpret(&self, statements: &[Stmt]) -> bool {
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                //e.report("");
                return false;
            }
        }
        true
    }

    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn execute(&self, statement: &Stmt) -> Result<(), LoxError> {
        statement.accept(self)
    }

    fn execute_block(&self, statements: &[Stmt], environment: Environment) -> Result<(), LoxError> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        let result = statements.iter().try_for_each(|s| self.execute(s));
        self.environment.replace(previous);
        result
    }

    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object, Object::Nil | Object::Bool(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    fn minus() -> Token {
        Token::new(TokenType::Minus, "-".to_string(), None, 1)
    }

    fn plus() -> Token {
        Token::new(TokenType::Plus, "+".to_string(), None, 1)
    }

    fn star() -> Token {
        Token::new(TokenType::Star, "*".to_string(), None, 1)
    }

    fn slash() -> Token {
        Token::new(TokenType::Slash, "/".to_string(), None, 1)
    }

    fn bang() -> Token {
        Token::new(TokenType::Bang, "!".to_string(), None, 1)
    }

    fn greater() -> Token {
        Token::new(TokenType::Greater, ">".to_string(), None, 1)
    }

    fn greater_equal() -> Token {
        Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1)
    }

    fn less() -> Token {
        Token::new(TokenType::Less, "<".to_string(), None, 1)
    }

    fn less_equal() -> Token {
        Token::new(TokenType::LessEqual, "<=".to_string(), None, 1)
    }

    fn bang_equal() -> Token {
        Token::new(TokenType::BangEqual, "!=".to_string(), None, 1)
    }

    fn equal_equal() -> Token {
        Token::new(TokenType::EqualEqual, "==".to_string(), None, 1)
    }

    fn number(n: i32) -> Object {
        Object::Num(n as f64)
    }

    fn boolean(b: bool) -> Object {
        Object::Bool(b)
    }

    fn string(s: &str) -> Object {
        Object::Str(s.to_string())
    }

    fn nil() -> Object {
        Object::Nil
    }

    fn run(expr: &Expr) -> Result<Object, LoxError> {
        let interpreter = Interpreter::new();
        interpreter.evaluate(expr)
    }

    #[test]
    fn unary_number() {
        let expr = Expr::Unary(UnaryExpr {
            operator: minus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = number(-10);
        let res = run(&expr).unwrap();

        assert_eq!(expected, res);
    }

    #[test]
    fn unary_double_number() {
        let expr = Expr::Unary(UnaryExpr {
            operator: minus(),
            right: Box::new(Expr::Unary(UnaryExpr {
                operator: minus(),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(number(10)),
                })),
            })),
        });
        let expected = number(10);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_star() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: star(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = number(100);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_minus() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: minus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = number(5);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_slash() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: slash(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = number(2);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_plus() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: plus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = number(15);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_plus_str() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string("hello")),
            })),
            operator: plus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string(" world!")),
            })),
        });
        let expected = string("hello world!");
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_arithmetic_error() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string("hello")),
            })),
            operator: plus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(3)),
            })),
        });
        let expected = Object::ArithmeticError;
        assert!(run(&expr).is_err());
    }

    #[test]
    fn unary_boolean() {
        let expr = Expr::Unary(UnaryExpr {
            operator: bang(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
        });
        let expected = Object::Bool(false);
        let res = run(&expr).unwrap();

        assert_eq!(expected, res);
    }

    #[test]
    fn unary_double_boolean() {
        let expr = Expr::Unary(UnaryExpr {
            operator: bang(),
            right: Box::new(Expr::Unary(UnaryExpr {
                operator: bang(),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(Object::Bool(true)),
                })),
            })),
        });
        let expected = Object::Bool(true);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_greater() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: greater(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_greater_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: greater(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_greater_equal() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: greater_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_greater_equal_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: greater_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_less_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: less(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_less() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: less(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_less_equal() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: less_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_less_equal_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
            operator: less_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_bang_equal_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
            operator: bang_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_bang_equal() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
            operator: bang_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_equal_equal_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
            operator: equal_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_equal_equal() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
            operator: equal_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_equal_equal_nil() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: Some(nil()) })),
            operator: equal_equal(),
            right: Box::new(Expr::Literal(LiteralExpr { value: Some(nil()) })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_equal_equal_nil_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(4)),
            })),
            operator: equal_equal(),
            right: Box::new(Expr::Literal(LiteralExpr { value: Some(nil()) })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_equal_equal_string() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string("hello")),
            })),
            operator: equal_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string("hello")),
            })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_equal_equal_string_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string("ello")),
            })),
            operator: equal_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string("hello")),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn test_var_stmt() {
        let interpreter = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 1);
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: Some(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        };
        assert!(interpreter.visit_var_stmt(&var_stmt).is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get(&name).unwrap(),
            Object::Num(10.0)
        );
    }

    #[test]
    fn test_var_stmt_undefined() {
        let interpreter = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 1);
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: None,
        };
        assert!(interpreter.visit_var_stmt(&var_stmt).is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get(&name).unwrap(),
            Object::Nil
        );
    }

    #[test]
    fn test_var_expr() {
        let interpreter = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 1);
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: Some(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        };
        assert!(interpreter.visit_var_stmt(&var_stmt).is_ok());
        let var_expression = VariableExpr { name: name.clone() };
        assert_eq!(
            interpreter.visit_variable_expr(&var_expression).unwrap(),
            Object::Num(10.0)
        );
    }

    #[test]
    fn test_var_expr_undefined() {
        let interpreter = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 1);
        let var_expression = VariableExpr { name: name.clone() };
        assert!(interpreter.visit_variable_expr(&var_expression).is_err());
    }

    #[test]
    fn assign_value_to_variable_undefined() {
        let mut e = Environment::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 1);
        assert!(e.assign(&name, Object::Nil).is_err());
    }

    #[test]
    fn reassign_value_to_existing_variable() {
        let mut e = Environment::new();
        let id = Token::new(TokenType::Identifier, "foo".to_string(), None, 1);
        e.define(&"foo".to_string(), Object::Num(10.0));
        assert_eq!(e.get(&id).unwrap(), Object::Num(10.0));
        assert!(e.assign(&id, Object::Num(100.0)).is_ok());
        assert_eq!(e.get(&id).unwrap(), Object::Num(100.0));
    }
}
