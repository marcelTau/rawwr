/// This file got generated by ./src/generate_ast.rs
/// Don't modify it
use crate::token::*;
use crate::scanner::*;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

pub struct LiteralExpr {
    value: Box<Literal>,
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, ScannerError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, ScannerError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, ScannerError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, ScannerError>;
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_unary_expr(self)
    }
}

