use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::callable::*;
use crate::class::*;
use crate::environment::*;
use crate::error::*;
use crate::expr::*;
use crate::function::*;
use crate::native_functions::*;
use crate::object::Object;
use crate::stmt::*;
use crate::token::*;

pub struct Interpreter {
    environment: RefCell<Rc<RefCell<Environment>>>,
    pub globals: Rc<RefCell<Environment>>,
    locals: RefCell<HashMap<Rc<Expr>, usize>>,
}

impl StmtVisitor<()> for Interpreter {
    fn visit_class_stmt(&self, _: Rc<Stmt>, stmt: &ClassStmt) -> Result<(), LoxResult> {
        let superclass = if let Some(superclass_expr) = &stmt.superclass {
            let superclass = self.evaluate(superclass_expr.clone())?;
            if let Object::Class(c) = superclass {
                Some(c)
            } else if let Expr::Variable(v) = superclass_expr.deref() {
                return Err(LoxResult::runtime_error(
                    &v.name,
                    "Superclass must be a class.",
                ));
            } else {
                panic!("Could not extract variable expr.");
            }
        } else {
            None
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(&stmt.name.lexeme, Object::Nil);

        let enclosing = if let Some(ref s) = superclass {
            let mut e = Environment::new_with_enclosing(self.environment.borrow().clone());
            e.define("super", Object::Class(s.clone()));
            Some(self.environment.replace(Rc::new(RefCell::new(e))))
        } else {
            None
        };

        let mut methods = HashMap::new();

        for method in stmt.methods.deref() {
            if let Stmt::Function(method) = method.deref() {
                let is_init = method.name.lexeme == "init";
                let function = Object::Func(Rc::new(Function::new(
                    method,
                    &self.environment.borrow(),
                    is_init,
                )));
                methods.insert(method.name.lexeme.clone(), function);
            } else {
                panic!("");
            };
        }
        let klass = Object::Class(Rc::new(Class::new(
            stmt.name.lexeme.clone(),
            superclass,
            methods,
        )));

        if let Some(prev) = enclosing {
            self.environment.replace(prev);
        }

        self.environment
            .borrow()
            .borrow_mut()
            .assign(&stmt.name, klass)?;
        Ok(())
    }

    fn visit_expression_stmt(&self, _: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        self.evaluate(stmt.expression.clone())?;
        Ok(())
    }

    fn visit_print_stmt(&self, _: Rc<Stmt>, stmt: &PrintStmt) -> Result<(), LoxResult> {
        let value = self.evaluate(stmt.expression.clone())?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&self, _: Rc<Stmt>, stmt: &VarStmt) -> Result<(), LoxResult> {
        let value: Object = if let Some(initializer) = &stmt.initializer {
            self.evaluate(initializer.clone())?
        } else {
            Object::Nil
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(&stmt.name.lexeme, value);
        Ok(())
    }

    fn visit_block_stmt(&self, _: Rc<Stmt>, stmt: &BlockStmt) -> Result<(), LoxResult> {
        let e = Environment::new_with_enclosing(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, e)
    }

    fn visit_if_stmt(&self, _: Rc<Stmt>, stmt: &IfStmt) -> Result<(), LoxResult> {
        if self.is_truthy(&self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.then_branch.clone())
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch.clone())
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&self, _: Rc<Stmt>, stmt: &WhileStmt) -> Result<(), LoxResult> {
        while self.is_truthy(&self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.body.clone())?;
        }
        Ok(())
    }

    fn visit_function_stmt(&self, _: Rc<Stmt>, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        let function = Function::new(stmt, &*self.environment.borrow(), false);
        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.as_str(), Object::Func(Rc::new(function)));
        Ok(())
    }

    fn visit_return_stmt(&self, wrapper: Rc<Stmt>, stmt: &ReturnStmt) -> Result<(), LoxResult> {
        if let Some(value) = &stmt.value {
            Err(LoxResult::return_value(self.evaluate(value.clone())?))
        } else {
            Err(LoxResult::return_value(Object::Nil))
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_super_expr(&self, wrapper: Rc<Expr>, expr: &SuperExpr) -> Result<Object, LoxResult> {
        let distance = *self.locals.borrow().get(&wrapper).unwrap();
        let superclass = self
            .environment
            .borrow()
            .borrow()
            .get_at(distance, "super");

        let object = self.environment.borrow().borrow().get_at(distance - 1, "this");

        if let Object::Class(sc) = superclass {
            let method = sc.find_method(expr.method.lexeme.clone());
            if let Some(method) = method {
                if let Object::Func(func) = method {
                    Ok(func.bind(&object))
                } else {
                    panic!("method was not found");
                }
            } else {
                Err(LoxResult::runtime_error(&expr.method, &format!("Undefined property '{}'.", expr.method.lexeme)))
            }
        } else {
            Err(LoxResult::runtime_error(&expr.method, &format!("Undefined property '{}'.", expr.method.lexeme)))
        }
    }

    fn visit_this_expr(&self, wrapper: Rc<Expr>, expr: &ThisExpr) -> Result<Object, LoxResult> {
        self.lookup_variable(&expr.keyword, wrapper)
    }
    fn visit_set_expr(&self, wrapper: Rc<Expr>, expr: &SetExpr) -> Result<Object, LoxResult> {
        let object = self.evaluate(expr.object.clone())?;

        if let Object::Instance(inst) = object {
            let value = self.evaluate(expr.value.clone())?;
            inst.set(&expr.name, &value)?;
            Ok(value)
        } else {
            Err(LoxResult::runtime_error(
                &expr.name,
                "Only instances have fields.",
            ))
        }
    }

    fn visit_get_expr(&self, _: Rc<Expr>, expr: &GetExpr) -> Result<Object, LoxResult> {
        let object = self.evaluate(expr.object.clone())?;
        if let Object::Instance(inst) = object {
            inst.get(&expr.name, &inst)
        } else {
            Err(LoxResult::runtime_error(
                &expr.name,
                "Only instances have properties.",
            ))
        }
    }

    fn visit_literal_expr(&self, _: Rc<Expr>, expr: &LiteralExpr) -> Result<Object, LoxResult> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, _: Rc<Expr>, expr: &GroupingExpr) -> Result<Object, LoxResult> {
        self.evaluate(expr.expression.clone())
    }

    fn visit_binary_expr(&self, _: Rc<Expr>, expr: &BinaryExpr) -> Result<Object, LoxResult> {
        let left = self.evaluate(expr.left.clone())?;
        let right = self.evaluate(expr.right.clone())?;

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
            Object::ArithmeticError | Object::DivByZeroError => Err(LoxResult::runtime_error(
                &expr.operator,
                format!("{}", result).as_str(),
            )),
            _ => Ok(result),
        }
    }

    fn visit_unary_expr(&self, _: Rc<Expr>, expr: &UnaryExpr) -> Result<Object, LoxResult> {
        let right = self.evaluate(expr.right.clone())?;

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Object::Num(n) => Ok(Object::Num(n * (-1_f64))),
                _ => Ok(Object::Nil),
            },
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
            _ => Err(LoxResult::runtime_error(&expr.operator, "Unreachable")),
        }
    }

    fn visit_variable_expr(
        &self,
        wrapper: Rc<Expr>,
        expr: &VariableExpr,
    ) -> Result<Object, LoxResult> {
        // self.environment.borrow().borrow().get(&expr.name)
        self.lookup_variable(&expr.name, wrapper)
    }

    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<Object, LoxResult> {
        let value = self.evaluate(expr.value.clone())?;

        if let Some(distance) = self.locals.borrow().get(&wrapper) {
            self.environment
                .borrow()
                .borrow_mut()
                .assign_at(*distance, &expr.name, &value)?;
        } else {
            self.globals
                .borrow_mut()
                .assign(&expr.name, value.clone())?;
        }

        Ok(value)
    }

    fn visit_logical_expr(&self, _: Rc<Expr>, expr: &LogicalExpr) -> Result<Object, LoxResult> {
        let left = self.evaluate(expr.left.clone())?;

        if expr.operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else if !self.is_truthy(&left) {
            return Ok(left);
        }
        self.evaluate(expr.right.clone())
    }

    fn visit_call_expr(&self, _: Rc<Expr>, expr: &CallExpr) -> Result<Object, LoxResult> {
        let callee = self.evaluate(expr.callee.clone())?;
        let mut arguments = Vec::new();

        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument.clone())?);
        }

        let (callfunc, klass): (Option<Rc<dyn LoxCallable>>, Option<Rc<Class>>) = match callee {
            Object::Func(f) => (Some(f), None),
            Object::Native(n) => (Some(n.func.clone()), None),
            Object::Class(c) => {
                let klass = Rc::clone(&c);
                (Some(c), Some(klass))
            }
            _ => (None, None),
        };

        if let Some(callfunc) = callfunc {
            if arguments.len() != callfunc.arity() {
                return Err(LoxResult::runtime_error(
                    &expr.paren,
                    &format!(
                        "Expected {} arguments but got {}.",
                        callfunc.arity(),
                        arguments.len()
                    ),
                ));
            }
            callfunc.call(self, arguments, klass)
        } else {
            Err(LoxResult::runtime_error(
                &expr.paren,
                "Can only call functions and classes",
            ))
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define(
            "clock",
            Object::Native(Rc::new(Native {
                func: Rc::new(NativeClock),
            })),
        );

        globals.borrow_mut().define(
            "num_to_str",
            Object::Native(Rc::new(Native {
                func: Rc::new(NativeNumToString),
            })),
        );

        // println!("{:?}", globals);

        Interpreter {
            globals: Rc::clone(&globals),
            environment: RefCell::new(Rc::clone(&globals)),
            locals: RefCell::new(HashMap::new()),
        }
    }

    pub fn interpret(&self, statements: Rc<Vec<Rc<Stmt>>>) -> bool {
        for statement in statements.deref() {
            if let Err(e) = self.execute(statement.clone()) {
                return false;
            }
        }
        true
    }

    fn evaluate(&self, expr: Rc<Expr>) -> Result<Object, LoxResult> {
        expr.accept(expr.clone(), self)
    }

    fn execute(&self, statement: Rc<Stmt>) -> Result<(), LoxResult> {
        statement.accept(statement.clone(), self)
    }

    pub fn execute_block(
        &self,
        statements: &Rc<Vec<Rc<Stmt>>>,
        environment: Environment,
    ) -> Result<(), LoxResult> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        let result = statements.iter().try_for_each(|s| self.execute(s.clone()));
        self.environment.replace(previous);
        result
    }

    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object, Object::Nil | Object::Bool(false))
    }

    pub fn resolve(&self, expr: Rc<Expr>, depth: usize) {
        self.locals.borrow_mut().insert(expr, depth);
    }

    fn lookup_variable(&self, name: &Token, expr: Rc<Expr>) -> Result<Object, LoxResult> {
        if let Some(distance) = self.locals.borrow().get(&expr) {
            Ok(self
                .environment
                .borrow()
                .borrow()
                .get_at(*distance, &name.lexeme.clone()))
        } else {
            self.globals.borrow().get(name)
        }
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

    fn run(expr: &Expr) -> Result<Object, LoxResult> {
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
            interpreter
                .environment
                .borrow()
                .borrow()
                .get(&name)
                .unwrap(),
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
            interpreter
                .environment
                .borrow()
                .borrow()
                .get(&name)
                .unwrap(),
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
