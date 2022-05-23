use crate::error::ScannerError;
use crate::expr::*;
use crate::object::Object;
use crate::token::TokenType;

struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, ScannerError> {
        //Ok(expr.value.unwrap().as_ref())
        Ok(Object::Nil)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, ScannerError> {
        Ok(Object::Nil)
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, ScannerError> {
        Ok(self.evaluate(&expr.expression)?)
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, ScannerError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Object::Num(n) => Ok(Object::Num(n * (-1 as f64))),
                _ => Ok(Object::Nil),
            },
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),

            _ => Err(ScannerError::Error {
                line: expr.operator.line,
                message: "This should be unreachable".to_string(),
            }),
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, ScannerError> {
        expr.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Nil | Object::Bool(false) => false,
            _ => true
        }
    }
}
