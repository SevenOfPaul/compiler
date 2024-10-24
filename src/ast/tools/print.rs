use crate::ast::expr::{Expr, Visitor};
use crate::ast::expr_visitor::ExprVisitor;

fn println(expr:&Expr){
    let v=ExprVisitor::new();
    println!("{}", v.visit(expr));
}