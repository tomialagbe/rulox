use crate::token::{Token, TokenLiteral};

use super::expr::Expr;

pub trait Visitor<R> {
    fn visit_binary_expr(&self, left: &Box<Expr>, right: &Box<Expr>, operator: &Token) -> R;
    fn visit_grouping(&self, expression: &Box<Expr>) -> R;
    fn visit_literal_expr(&self, value: &Option<TokenLiteral>) -> R;
    fn visit_unary_expr(&self, operator: &Token, right: &Box<Expr>) -> R;
}
