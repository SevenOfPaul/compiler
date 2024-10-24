use crate::ast::token::object::Object;
use crate::ast::token::token_type::Token_type;
use std::fmt::Binary;
pub(crate) enum Expr {
    Binary {
        operator: Token_type,
        l_expression: Box<Expr>,
        r_expression: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        val: Option<Object>,
    },
    Unary {
        operator: Token_type,
        r_expression: Box<Expr>,
    },
}

pub trait Visitor<T> {
    fn visit(&self, expr: &Expr) -> T;
}
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit(&self)
    }
}
