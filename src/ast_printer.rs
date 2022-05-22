use crate::expr::*;
use crate::scanner::ScannerError;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String, ScannerError> {
        expr.accept(self)
    }
    fn parenthesize(&self, name: &String, exprs: &[&Box<Expr>]) -> Result<String, ScannerError> {
        let mut builder = format!("({name}");
        for expr in exprs {
            builder = format!("{builder} {}", expr.accept(self)?);
        };
        builder = format!("{builder})");

        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, ScannerError> {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, ScannerError> {
        self.parenthesize(&"group".to_string(), &vec![&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, ScannerError> {
        Ok(expr.value.as_string())
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, ScannerError> {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.right])
    }

}
