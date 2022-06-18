#![allow(unused_imports)]
use crate::token::*;
use crate::object::*;
use crate::error::*;
use std::rc::Rc;

pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

impl Expr {
    pub fn accept<T>(&self, wrapper: &Rc<Expr>, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Expr::Assign(x) => visitor.visit_assign_expr(wrapper, &x),
            Expr::Binary(x) => visitor.visit_binary_expr(wrapper, &x),
            Expr::Call(x) => visitor.visit_call_expr(wrapper, &x),
            Expr::Grouping(x) => visitor.visit_grouping_expr(wrapper, &x),
            Expr::Literal(x) => visitor.visit_literal_expr(wrapper, &x),
            Expr::Logical(x) => visitor.visit_logical_expr(wrapper, &x),
            Expr::Unary(x) => visitor.visit_unary_expr(wrapper, &x),
            Expr::Variable(x) => visitor.visit_variable_expr(wrapper, &x),
        }
    }
}
pub struct AssignExpr {
    pub name: Token,
    pub value: Rc<Expr>,
}

pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

pub struct CallExpr {
    pub callee: Rc<Expr>,
    pub paren: Token,
    pub arguments: Vec<Rc<Expr>>,
}

pub struct GroupingExpr {
    pub expression: Rc<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Object>,
}

pub struct LogicalExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<Expr>,
}

pub struct VariableExpr {
    pub name: Token,
}

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&self, wrapper: &Rc<Expr>, expr: &AssignExpr) -> Result<T, LoxResult>;
    fn visit_binary_expr(&self, wrapper: &Rc<Expr>, expr: &BinaryExpr) -> Result<T, LoxResult>;
    fn visit_call_expr(&self, wrapper: &Rc<Expr>, expr: &CallExpr) -> Result<T, LoxResult>;
    fn visit_grouping_expr(&self, wrapper: &Rc<Expr>, expr: &GroupingExpr) -> Result<T, LoxResult>;
    fn visit_literal_expr(&self, wrapper: &Rc<Expr>, expr: &LiteralExpr) -> Result<T, LoxResult>;
    fn visit_logical_expr(&self, wrapper: &Rc<Expr>, expr: &LogicalExpr) -> Result<T, LoxResult>;
    fn visit_unary_expr(&self, wrapper: &Rc<Expr>, expr: &UnaryExpr) -> Result<T, LoxResult>;
    fn visit_variable_expr(&self, wrapper: &Rc<Expr>, expr: &VariableExpr) -> Result<T, LoxResult>;
}

