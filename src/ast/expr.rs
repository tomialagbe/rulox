use crate::token::{Token, TokenLiteral};

use super::visitor::Visitor;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Option<TokenLiteral>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        use self::Expr::*;
        match self {
            Binary {
                left,
                right,
                operator,
            } => visitor.visit_binary_expr(left, right, operator),
            Grouping { expression } => visitor.visit_grouping(expression),
            Literal { value } => visitor.visit_literal_expr(value),
            Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}