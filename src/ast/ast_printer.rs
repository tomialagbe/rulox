use crate::token::{Token, TokenLiteral};

use super::{expr::Expr, visitor::Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, mut expr: Expr) -> String {
        expr.accept::<String>(self)
    }

    fn parenthesize(&self, name: &str, exprs: &mut [&Expr]) -> String {
        let mut sb = String::new();
        sb.push_str("(");
        sb.push_str(name);
        for exp in exprs {
            sb.push_str(" ");
            sb.push_str(exp.accept::<String>(self).as_str());
        }
        sb.push_str(")");

        sb
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Box<Expr>, right: &Box<Expr>, operator: &Token) -> String {
        self.parenthesize(&operator.lexeme, &mut [left, right])
    }

    fn visit_grouping(&self, expression: &Box<Expr>) -> String {
        self.parenthesize("group", &mut [expression])
    }

    fn visit_literal_expr(&self, value: &Option<TokenLiteral>) -> String {
        let Some(literal) = value else {
            return "nil".to_string();
        };

        if literal.num_val().is_some() {
            return format!("{}", literal.num_val().unwrap());
        } else {
            return format!("{}", literal.string_val().unwrap());
        }
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Box<Expr>) -> String {
        self.parenthesize(&operator.lexeme, &mut [right])
    }
}
