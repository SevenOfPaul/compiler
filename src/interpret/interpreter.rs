use crate::ast::expr::{Expr, Visitor};
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type::NIL;
use crate::interpret::error::Run_Err;
use crate::interpret::value::Value;

struct Interpreter {}
impl Visitor<Result<Value,Run_Err>> for Interpreter {
    fn visit(&self, expr: &Expr) -> Result<Value,Run_Err> {
     match expr {
         Expr::Literal {val}=>{
             // Ok(Value::)
         },
             _=>{
                 Err(Run_Err::new(Token::new(NIL,"")))
             }
     }
    }
}
impl Interpreter {
    fn visit_literal(&self,expr: &Expr){
        // expr.val
    }
}