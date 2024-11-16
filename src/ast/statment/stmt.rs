use crate::ast::expression::expr::Expr;
use crate::ast::token::token::Token;

pub (crate) enum Stmt {
    Expression { expr: Box<Expr> },
    Print { expr: Box<Expr> },
}
