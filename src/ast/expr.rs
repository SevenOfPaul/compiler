use crate::ast::token::object::Object;
use std::fmt::Binary;
use crate::ast::token::token::Token;

#[derive(Debug)]
pub(crate) enum Expr {
    Binary {
        operator: Token,
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
        operator: Token,
        r_expression: Box<Expr>,
    },
}

pub trait Visitor<T> {
    fn visit(&self, expr: &Expr) -> T;
}
impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit(self)
    }
}
/*
impl Expr{
pub fn accept<T>(&self,visitor:&dyn Visitor<T>)->T{
visitor.visit(self)
}
pub fn accept<T,F>(&self,vsiit:&dyn visit:F)->T
where F:Fn(expr:&Expr)->T{
vsiit(self)
}
*/
