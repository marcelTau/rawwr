use crate::error::ScannerError;
use crate::expr::*;
use crate::object::Object;
use crate::token::TokenType;

struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, ScannerError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, ScannerError> {
        Ok(self.evaluate(&expr.expression)?)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, ScannerError> {
        Ok(Object::Nil)
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

#[cfg(test)]
mod tests {
    use crate::token::Token;
    use super::*;

    fn run(expr: &Expr) -> Object {
        let interpreter = Interpreter {};
        interpreter.evaluate(expr).unwrap()
    }

    #[test]
    fn unary_number() {
        let expr = Expr::Unary(UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(8 as f64)),
            })
            )
        });
        let expected = Object::Num(-8 as f64);
        let res = run(&expr);

        assert_eq!(expected, res);
    }

    #[test]
    fn unary_double_number() {
        let expr = Expr::Unary(UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(
                Expr::Unary(UnaryExpr {
                    operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                    right: Box::new(Expr::Literal(LiteralExpr {
                        value: Some(Object::Num(10 as f64)),
                    }),
                )}),
            )
        });
        let expected = Object::Num(10 as f64);
        let res = run(&expr);

        assert_eq!(expected, res);
    }

    #[test]
    fn unary_boolean() {
        let expr = Expr::Unary(UnaryExpr {
            operator: Token::new(TokenType::Bang, "!".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })
            )
        });
        let expected = Object::Bool(false);
        let res = run(&expr);

        assert_eq!(expected, res);
    }

    #[test]
    fn unary_double_boolean() {
        let expr = Expr::Unary(UnaryExpr {
            operator: Token::new(TokenType::Bang, "!".to_string(), None, 1),
            right: Box::new(
                Expr::Unary(UnaryExpr {
                    operator: Token::new(TokenType::Bang, "!".to_string(), None, 1),
                    right: Box::new(Expr::Literal(LiteralExpr {
                        value: Some(Object::Bool(true)),
                    }),
                )}),
            )
        });
        let expected = Object::Bool(true);
        let res = run(&expr);

        assert_eq!(expected, res);
    }
}










