use crate::ast::expr::{Expr, Visitor};
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type;
use crate::ast::token::token_type::Token_Type::NIL;
use crate::interpret::error::Run_Err;
use crate::interpret::value::Value;

struct Interpreter {}
impl Visitor<Result<Value, Run_Err>> for Interpreter {
    fn visit_binary(
        &mut self,
        operator: &Token,
        l_expression: &Expr,
        r_expression: &Expr,
    ) -> Result<Value, Run_Err> {
        todo!()
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<Value, Run_Err> {
        self.evaluate(expr)
    }

    fn visit_literal(&mut self, value: &Object) -> Result<Value, Run_Err> {
        Ok(match value {
            Object::str(s) => Value::str(s.clone()),
            Object::num(n) => Value::num(*n),
            Object::bool(b) => Value::bool(*b),
            _ => Value::nil,
        })
    }
    fn visit_unary(&mut self, operator: &Token, r_expression: &Expr) -> Result<Value, Run_Err> {
        let r_value = self.evaluate(r_expression);
        match operator.token_type {
            Token_Type::MINUS => Ok(-r_value?),
            Token_Type::BANG=>Ok(Value::bool(!self.is_truthy(r_value))),
            _ => Ok(Value::nil),
        }
    }
}
impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, Run_Err> {
        expr.accept(self)
    }
    fn is_truthy(&self, value: Result<Value,Run_Err>) -> bool {
      if let Ok(v)=value{
           match v{
               Value::nil => false,
               Value::num(v) => v == 0.0,
               Value::str(s) => false,
               Value::bool(b) => b,
           }
       }else{
           false
       }
    }
}
