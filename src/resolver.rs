use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::error::*;
use crate::expr::*;
use crate::interpreter::*;
use crate::stmt::*;
use crate::token::*;

pub struct Resolver {
    interpreter: Interpreter,
    scopes: RefCell<Vec<HashMap<String, bool>>>,
}

impl StmtVisitor<()> for Resolver {
    fn visit_block_stmt(&self, _: &Rc<Stmt>, stmt: &BlockStmt) -> Result<(), LoxResult> {
        self.begin_scope();
        self.resolve(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }
    fn visit_expression_stmt(&self, _: &Rc<Stmt>, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_function_stmt(&self, _: &Rc<Stmt>, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_if_stmt(&self, _: &Rc<Stmt>, stmt: &IfStmt) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_print_stmt(&self, _: &Rc<Stmt>, stmt: &PrintStmt) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_return_stmt(&self, _: &Rc<Stmt>, stmt: &ReturnStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_var_stmt(&self, _: &Rc<Stmt>, stmt: &VarStmt) -> Result<(), LoxResult> {
        self.declare(&stmt.name);

        if let Some(initializer) = &stmt.initializer {
            self.resolve_expr(&initializer)?;
        }

        self.define(&stmt.name);

        Ok(())
    }

    fn visit_while_stmt(&self, _: &Rc<Stmt>, stmt: &WhileStmt) -> Result<(), LoxResult> {
        Ok(())
    }
}

impl Resolver {
    fn new(interpreter: Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: RefCell::new(Vec::new()),
        }
    }
    fn resolve(&self, statements: &Rc<Vec<Rc<Stmt>>>) -> Result<(), LoxResult> {
        for statement in statements.deref() {
            self.resolve_stmt(&statement)?;
        }

        Ok(())
    }

    fn resolve_stmt(&self, stmt: &Rc<Stmt>) -> Result<(), LoxResult> {
        stmt.accept(stmt, self)
    }

    fn begin_scope(&self) {
        self.scopes
            .borrow_mut()
            .push(HashMap::<String, bool>::new());
    }

    fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&self, name: &Token) {
        if !self.scopes.borrow().is_empty() {
            self.scopes
                .borrow_mut()
                .last_mut()
                .unwrap()
                .insert(name.lexeme.clone(), false);
        }
    }

    fn define(&self, name: &Token) {
        if !self.scopes.borrow().is_empty() {
            self.scopes
                .borrow_mut()
                .last_mut()
                .unwrap()
                .insert(name.lexeme.clone(), true);
        }
    }

    // -----------------------------------------------------------------------------

    fn resolve_expr(&self, expr: &Rc<Expr>) -> Result<(), LoxResult> {
        expr.accept(expr, self)
    }

    fn resolve_local(&self, expr: &Rc<Expr>, name: &Token) {
        for (scope, map) in self.scopes.borrow().iter().rev().enumerate() {
            if map.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr.clone(), scope); // @todo no clone here probably
                return;
            }
        }
    }
}

impl ExprVisitor<()> for Resolver {
    fn visit_literal_expr(&self, _: &Rc<Expr>, expr: &LiteralExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_grouping_expr(&self, _: &Rc<Expr>, expr: &GroupingExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_binary_expr(&self, _: &Rc<Expr>, expr: &BinaryExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_unary_expr(&self, _: &Rc<Expr>, expr: &UnaryExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_variable_expr(&self, wrapper: &Rc<Expr>, expr: &VariableExpr) -> Result<(), LoxResult> {
        if !self.scopes.borrow().is_empty()
            && self
                .scopes
                .borrow()
                .last()
                .unwrap()
                .get(&expr.name.lexeme)
                .unwrap()
                == &false
        {
            Err(LoxResult::runtime_error(&expr.name, "Can't read local variable in it's own initializer"))
        } else {
            self.resolve_local(wrapper, &expr.name);
            Ok(())
        }

    }
    fn visit_assign_expr(&self, _: &Rc<Expr>, expr: &AssignExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_logical_expr(&self, _: &Rc<Expr>, expr: &LogicalExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_call_expr(&self, _: &Rc<Expr>, expr: &CallExpr) -> Result<(), LoxResult> {
        Ok(())
    }
}
