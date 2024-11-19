use crate::ast::expression::expr::Expr;
use crate::interpret::interpreter::Interpreter;
use crate::interpret::value::Value;
#[derive(Clone,Debug)]
pub (crate) enum Stmt {
    Expression { expr: Box<Expr> },
    Print { expr: Box<Expr> },
}
pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr);
    fn visit_print(&mut self, expr: &Expr);
}

impl Stmt{
    pub(crate) fn accept<T>(&self, visitor: &mut dyn Visitor<T>){
          match self {
            Stmt::Expression {expr}=>{
                visitor.visit_expr(expr.as_ref());
            }
              Stmt::Print {expr}=>{
                  visitor.visit_print(expr.as_ref());
              }
          }
    }
}
