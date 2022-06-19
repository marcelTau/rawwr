#![allow(unused_imports)]
use crate::token::*;
use crate::object::*;
use crate::error::*;
use std::rc::Rc;
use std::hash::{Hash, Hasher};

pub enum Expr {
    Assign(Rc<AssignExpr>),
    Binary(Rc<BinaryExpr>),
    Call(Rc<CallExpr>),
    Get(Rc<GetExpr>),
    Grouping(Rc<GroupingExpr>),
    Literal(Rc<LiteralExpr>),
    Logical(Rc<LogicalExpr>),
    Unary(Rc<UnaryExpr>),
    Variable(Rc<VariableExpr>),
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
                  (Expr::Assign(a), Expr::Assign(b)) => Rc::ptr_eq(a, b),
                  (Expr::Binary(a), Expr::Binary(b)) => Rc::ptr_eq(a, b),
                  (Expr::Call(a), Expr::Call(b)) => Rc::ptr_eq(a, b),
                  (Expr::Get(a), Expr::Get(b)) => Rc::ptr_eq(a, b),
                  (Expr::Grouping(a), Expr::Grouping(b)) => Rc::ptr_eq(a, b),
                  (Expr::Literal(a), Expr::Literal(b)) => Rc::ptr_eq(a, b),
                  (Expr::Logical(a), Expr::Logical(b)) => Rc::ptr_eq(a, b),
                  (Expr::Unary(a), Expr::Unary(b)) => Rc::ptr_eq(a, b),
                  (Expr::Variable(a), Expr::Variable(b)) => Rc::ptr_eq(a, b),
                  _ => false,
        }
    }
}

impl Eq for Expr{}

impl Hash for Expr {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        match self {
        Expr::Assign(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Expr::Binary(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Expr::Call(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Expr::Get(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Expr::Grouping(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Expr::Literal(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Expr::Logical(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Expr::Unary(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Expr::Variable(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
    }
    }
}
impl Expr {
    pub fn accept<T>(&self, wrapper: Rc<Expr>, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Expr::Assign(x) => visitor.visit_assign_expr(wrapper, x),
            Expr::Binary(x) => visitor.visit_binary_expr(wrapper, x),
            Expr::Call(x) => visitor.visit_call_expr(wrapper, x),
            Expr::Get(x) => visitor.visit_get_expr(wrapper, x),
            Expr::Grouping(x) => visitor.visit_grouping_expr(wrapper, x),
            Expr::Literal(x) => visitor.visit_literal_expr(wrapper, x),
            Expr::Logical(x) => visitor.visit_logical_expr(wrapper, x),
            Expr::Unary(x) => visitor.visit_unary_expr(wrapper, x),
            Expr::Variable(x) => visitor.visit_variable_expr(wrapper, x),
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

pub struct GetExpr {
    pub object: Rc<Expr>,
    pub name: Token,
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
    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<T, LoxResult>;
    fn visit_binary_expr(&self, wrapper: Rc<Expr>, expr: &BinaryExpr) -> Result<T, LoxResult>;
    fn visit_call_expr(&self, wrapper: Rc<Expr>, expr: &CallExpr) -> Result<T, LoxResult>;
    fn visit_get_expr(&self, wrapper: Rc<Expr>, expr: &GetExpr) -> Result<T, LoxResult>;
    fn visit_grouping_expr(&self, wrapper: Rc<Expr>, expr: &GroupingExpr) -> Result<T, LoxResult>;
    fn visit_literal_expr(&self, wrapper: Rc<Expr>, expr: &LiteralExpr) -> Result<T, LoxResult>;
    fn visit_logical_expr(&self, wrapper: Rc<Expr>, expr: &LogicalExpr) -> Result<T, LoxResult>;
    fn visit_unary_expr(&self, wrapper: Rc<Expr>, expr: &UnaryExpr) -> Result<T, LoxResult>;
    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> Result<T, LoxResult>;
}

