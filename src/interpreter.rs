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
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        let result = match expr.operator.token_type {
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Greater => Object::Bool(left > right),
            TokenType::GreaterEqual => Object::Bool(left >= right),
            TokenType::Less => Object::Bool(left < right),
            TokenType::LessEqual => Object::Bool(left <= right),
            _ => unreachable!(),
        };

        if result == Object::ArithmeticError {
            Err(ScannerError::Error {
                line: expr.operator.line,
                message: "Illegeal Expression".to_string(),
            })
        } else {
            Ok(result)
        }
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
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    fn minus() -> Token {
        Token::new(TokenType::Minus, "-".to_string(), None, 1)
    }

    fn plus() -> Token {
        Token::new(TokenType::Plus, "+".to_string(), None, 1)
    }

    fn star() -> Token {
        Token::new(TokenType::Star, "*".to_string(), None, 1)
    }

    fn slash() -> Token {
        Token::new(TokenType::Slash, "/".to_string(), None, 1)
    }

    fn bang() -> Token {
        Token::new(TokenType::Bang, "!".to_string(), None, 1)
    }

    fn greater() -> Token {
        Token::new(TokenType::Greater, ">".to_string(), None, 1)
    }

    fn greater_equal() -> Token {
        Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1)
    }

    fn less() -> Token {
        Token::new(TokenType::Less, "<".to_string(), None, 1)
    }

    fn less_equal() -> Token {
        Token::new(TokenType::LessEqual, "<=".to_string(), None, 1)
    }

    fn number(n: i32) -> Object {
        Object::Num(n as f64)
    }

    fn boolean(b: bool) -> Object {
        Object::Bool(b)
    }

    fn string(s: &str) -> Object {
        Object::Str(s.to_string())
    }

    fn run(expr: &Expr) -> Result<Object, ScannerError> {
        let interpreter = Interpreter {};
        interpreter.evaluate(expr)
    }

    #[test]
    fn unary_number() {
        let expr = Expr::Unary(UnaryExpr {
            operator: minus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = number(-10);
        let res = run(&expr).unwrap();

        assert_eq!(expected, res);
    }

    #[test]
    fn unary_double_number() {
        let expr = Expr::Unary(UnaryExpr {
            operator: minus(),
            right: Box::new(Expr::Unary(UnaryExpr {
                operator: minus(),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(number(10)),
                })),
            })),
        });
        let expected = number(10);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_star() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: star(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = number(100);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_minus() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: minus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = number(5);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_slash() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: slash(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = number(2);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_plus() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: plus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = number(15);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_plus_str() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string("hello")),
            })),
            operator: plus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string(" world!")),
            })),
        });
        let expected = string("hello world!");
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn binary_arithmetic_error() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(string("hello")),
            })),
            operator: plus(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(3)),
            })),
        });
        let expected = Object::ArithmeticError;
        assert!(run(&expr).is_err());
    }

    #[test]
    fn unary_boolean() {
        let expr = Expr::Unary(UnaryExpr {
            operator: bang(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
        });
        let expected = Object::Bool(false);
        let res = run(&expr).unwrap();

        assert_eq!(expected, res);
    }

    #[test]
    fn unary_double_boolean() {
        let expr = Expr::Unary(UnaryExpr {
            operator: bang(),
            right: Box::new(Expr::Unary(UnaryExpr {
                operator: bang(),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(Object::Bool(true)),
                })),
            })),
        });
        let expected = Object::Bool(true);
        let res = run(&expr).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn binary_greater() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: greater(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
        
    }

    #[test]
    fn binary_greater_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: greater(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
        
    }

    #[test]
    fn binary_greater_equal() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: greater_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
        
    }

    #[test]
    fn binary_greater_equal_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: greater_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
        
    }

    #[test]
    fn binary_less_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: less(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(5)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
        
    }
    #[test]
    fn binary_less() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: less(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
        
    }
    #[test]
    fn binary_less_equal() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
            operator: less_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(true);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
        
    }
    #[test]
    fn binary_less_equal_fail() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(11)),
            })),
            operator: less_equal(),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(number(10)),
            })),
        });
        let expected = boolean(false);
        let res = run(&expr);
        assert_eq!(expected, res.unwrap());
        
    }
}
