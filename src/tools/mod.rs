use crate::ast::expr::{Expr, Visitor};
use crate::ast::expr_visitor::ExprVisitor;

pub(crate) fn println(expr:&Expr){
    let mut ev =ExprVisitor::new();
      println!("{}",expr.accept(&mut ev));
}