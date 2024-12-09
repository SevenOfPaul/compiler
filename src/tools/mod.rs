use crate::ast::expression::expr::{Expr, Visitor};
use crate::ast::expression::expr_visitor::ExprVisitor;
use crate::interpret::value::Value;

pub(crate) fn printf(v:Value){
    match v {
        Value::Num(num) => println!("{}", num),
        Value::Str(s)=>println!("{}", s),
        Value::Bool(b)=>println!("{}", b),
        _=>println!("{:?}",Value::Nil)
    }
}