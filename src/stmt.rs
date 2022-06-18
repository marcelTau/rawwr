#![allow(unused_imports)]
use crate::token::*;
use crate::object::*;
use crate::error::*;
use crate::expr::*;
use std::rc::Rc;

pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Function(FunctionStmt),
    If(IfStmt),
    Print(PrintStmt),
    Return(ReturnStmt),
    Var(VarStmt),
    While(WhileStmt),
}

impl Stmt {
    pub fn accept<T>(&self, wrapper: &Rc<Stmt>, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Stmt::Block(x) => visitor.visit_block_stmt(wrapper, &x),
            Stmt::Expression(x) => visitor.visit_expression_stmt(wrapper, &x),
            Stmt::Function(x) => visitor.visit_function_stmt(wrapper, &x),
            Stmt::If(x) => visitor.visit_if_stmt(wrapper, &x),
            Stmt::Print(x) => visitor.visit_print_stmt(wrapper, &x),
            Stmt::Return(x) => visitor.visit_return_stmt(wrapper, &x),
            Stmt::Var(x) => visitor.visit_var_stmt(wrapper, &x),
            Stmt::While(x) => visitor.visit_while_stmt(wrapper, &x),
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
    fn visit_block_stmt(&self, wrapper: &Rc<Stmt>, stmt: &BlockStmt) -> Result<T, LoxResult>;
    fn visit_expression_stmt(&self, wrapper: &Rc<Stmt>, stmt: &ExpressionStmt) -> Result<T, LoxResult>;
    fn visit_function_stmt(&self, wrapper: &Rc<Stmt>, stmt: &FunctionStmt) -> Result<T, LoxResult>;
    fn visit_if_stmt(&self, wrapper: &Rc<Stmt>, stmt: &IfStmt) -> Result<T, LoxResult>;
    fn visit_print_stmt(&self, wrapper: &Rc<Stmt>, stmt: &PrintStmt) -> Result<T, LoxResult>;
    fn visit_return_stmt(&self, wrapper: &Rc<Stmt>, stmt: &ReturnStmt) -> Result<T, LoxResult>;
    fn visit_var_stmt(&self, wrapper: &Rc<Stmt>, stmt: &VarStmt) -> Result<T, LoxResult>;
    fn visit_while_stmt(&self, wrapper: &Rc<Stmt>, stmt: &WhileStmt) -> Result<T, LoxResult>;
}

