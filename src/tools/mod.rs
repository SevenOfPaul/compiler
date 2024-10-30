use crate::ast::expr::{Expr, Visitor};
use crate::ast::expr_visitor::ExprVisitor;

pub(crate) fn println(expr:&Expr){
    let  ev =ExprVisitor::new();
    println!("{}",ev.visit(expr));
}