use std::cell::RefCell;
use std::collections::HashMap;

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
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxResult> {
        self.begin_scope();
        self.resolve(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_return_stmt(&self, stmt: &ReturnStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxResult> {
        self.declare(&stmt.name);

        if let Some(initializer) = &stmt.initializer {
            self.resolve_expr(&initializer)?;
        }

        self.define(&stmt.name);

        Ok(())
    }

    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), LoxResult> {
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
    fn resolve(&self, statements: &[Stmt]) -> Result<(), LoxResult> {
        for statement in statements {
            self.resolve_stmt(&statement)?;
        }

        Ok(())
    }

    fn resolve_stmt(&self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
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

    fn resolve_expr(&self, expr: &Expr) -> Result<(), LoxResult> {
        expr.accept(self)
    }
}

impl ExprVisitor<()> for Resolver {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<(), LoxResult> {
        Ok(())
    }
    fn visit_call_expr(&self, expr: &CallExpr) -> Result<(), LoxResult> {
        Ok(())
    }
}
