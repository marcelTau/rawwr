#![allow(unused_imports)]
use crate::token::*;
use crate::object::*;
use crate::error::*;
use crate::expr::*;

pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block(x) => x.accept(visitor),
            Stmt::Expression(x) => x.accept(visitor),
            Stmt::Print(x) => x.accept(visitor),
            Stmt::Var(x) => x.accept(visitor),
        }
    }
}
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<T, LoxError>;
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<T, LoxError>;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<T, LoxError>;
    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<T, LoxError>;
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_block_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_var_stmt(self)
    }
}

