#![allow(unused_imports)]
use crate::token::*;
use crate::object::*;
use crate::error::*;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use crate::expr::*;

pub enum Stmt {
    Block(Rc<BlockStmt>),
    Expression(Rc<ExpressionStmt>),
    Function(Rc<FunctionStmt>),
    If(Rc<IfStmt>),
    Print(Rc<PrintStmt>),
    Return(Rc<ReturnStmt>),
    Var(Rc<VarStmt>),
    While(Rc<WhileStmt>),
}

impl PartialEq for Stmt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
                  (Stmt::Block(a), Stmt::Block(b)) => Rc::ptr_eq(a, b),
                  (Stmt::Expression(a), Stmt::Expression(b)) => Rc::ptr_eq(a, b),
                  (Stmt::Function(a), Stmt::Function(b)) => Rc::ptr_eq(a, b),
                  (Stmt::If(a), Stmt::If(b)) => Rc::ptr_eq(a, b),
                  (Stmt::Print(a), Stmt::Print(b)) => Rc::ptr_eq(a, b),
                  (Stmt::Return(a), Stmt::Return(b)) => Rc::ptr_eq(a, b),
                  (Stmt::Var(a), Stmt::Var(b)) => Rc::ptr_eq(a, b),
                  (Stmt::While(a), Stmt::While(b)) => Rc::ptr_eq(a, b),
                  _ => false,
        }
    }
}

impl Eq for Stmt{}

impl Hash for Stmt {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        match self {
        Stmt::Block(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Stmt::Expression(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Stmt::Function(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Stmt::If(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Stmt::Print(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Stmt::Return(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Stmt::Var(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
        Stmt::While(a) => {
            hasher.write_usize(Rc::as_ptr(a) as usize);
        },
    }
    }
}
impl Stmt {
    pub fn accept<T>(&self, wrapper: Rc<Stmt>, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Stmt::Block(x) => visitor.visit_block_stmt(wrapper, x),
            Stmt::Expression(x) => visitor.visit_expression_stmt(wrapper, x),
            Stmt::Function(x) => visitor.visit_function_stmt(wrapper, x),
            Stmt::If(x) => visitor.visit_if_stmt(wrapper, x),
            Stmt::Print(x) => visitor.visit_print_stmt(wrapper, x),
            Stmt::Return(x) => visitor.visit_return_stmt(wrapper, x),
            Stmt::Var(x) => visitor.visit_var_stmt(wrapper, x),
            Stmt::While(x) => visitor.visit_while_stmt(wrapper, x),
        }
    }
}
pub struct BlockStmt {
    pub statements: Rc<Vec<Rc<Stmt>>>,
}

pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

pub struct FunctionStmt {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Rc<Stmt>>>,
}

pub struct IfStmt {
    pub condition: Rc<Expr>,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}

pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Rc<Expr>>,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Rc<Expr>>,
}

pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Rc<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, wrapper: Rc<Stmt>, stmt: &BlockStmt) -> Result<T, LoxResult>;
    fn visit_expression_stmt(&self, wrapper: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<T, LoxResult>;
    fn visit_function_stmt(&self, wrapper: Rc<Stmt>, stmt: &FunctionStmt) -> Result<T, LoxResult>;
    fn visit_if_stmt(&self, wrapper: Rc<Stmt>, stmt: &IfStmt) -> Result<T, LoxResult>;
    fn visit_print_stmt(&self, wrapper: Rc<Stmt>, stmt: &PrintStmt) -> Result<T, LoxResult>;
    fn visit_return_stmt(&self, wrapper: Rc<Stmt>, stmt: &ReturnStmt) -> Result<T, LoxResult>;
    fn visit_var_stmt(&self, wrapper: Rc<Stmt>, stmt: &VarStmt) -> Result<T, LoxResult>;
    fn visit_while_stmt(&self, wrapper: Rc<Stmt>, stmt: &WhileStmt) -> Result<T, LoxResult>;
}

