use crate::ast::expression::expr::{Expr, Visitor};
use crate::ast::expression::expr_visitor::ExprVisitor;
use crate::interpret::value::Value;

pub(crate) fn printf(v:Value){
    match v {
        Value::num(num) => println!("{}",num),
        Value::str(s)=>println!("{}",s),
        Value::bool(b)=>println!("{}",b),
        _=>println!("{:?}",Value::nil)
    }
}