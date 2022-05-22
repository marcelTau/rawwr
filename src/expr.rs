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

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        match self {
            Expr::Binary(x) => x.accept(visitor),
            Expr::Grouping(x) => x.accept(visitor),
            Expr::Literal(x) => x.accept(visitor),
            Expr::Unary(x) => x.accept(visitor),
        }
    }
}
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Literal,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, ScannerError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, ScannerError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, ScannerError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, ScannerError>;
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_unary_expr(self)
    }
}

