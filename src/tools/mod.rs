use crate::ast::expression::expr::{Expr, Visitor};
use crate::ast::expression::expr_visitor::ExprVisitor;

pub(crate) fn println(expr:&Expr){
    let mut ev =ExprVisitor::new();
      println!("{}",expr.accept(&mut ev));
}