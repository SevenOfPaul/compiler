use crate::ast::expression::expr::{Expr, Visitor};
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type;
use crate::interpret::error::Run_Err;
use crate::interpret::value::Value;
use crate::error;

pub (crate) struct Interpreter {}
impl Visitor<Result<Value, Run_Err>> for Interpreter {
    fn visit_binary(
        &mut self,
        operator: &Token,
        l_expression: &Expr,
        r_expression: &Expr,
    ) -> Result<Value, Run_Err> {
        let l = self.evaluate(l_expression)?;
        let r = self.evaluate(r_expression)?;
        Ok(match operator.token_type {
            Token_Type::PLUS => l + r,
            Token_Type::MINUS => l + r,
            Token_Type::STAR => l * r,
            Token_Type::SLASH => l / r,
            Token_Type::GREATER=>Value::bool(l>r),
             Token_Type::LESS=>Value::bool(l<r),
            Token_Type::GREATER_EQUAL=>Value::bool(l>=r),
            Token_Type::EQUAL_EQUAL=>Value::bool(l==r),
            Token_Type::BANG_EQUAL=>Value::bool(l!=r),
            Token_Type::LESS_EQUAL=>Value::bool(l<=r),
            _ => panic!("操作符错误"),
        })
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
            Token_Type::BANG => Ok(!r_value?),
            _ => Ok(Value::nil),
        }
    }
}
impl Interpreter {
    pub(crate) fn new()->Self{
        Self{}
    }
   pub(crate) fn interpret(&mut self,expr:&Expr)->Value{
        let res = self.evaluate(expr);
        res.unwrap_or_else(|e| {
            error::log(0, "", &*e.mes);
            Value::nil
        })
    }
    pub(crate) fn evaluate(&mut self, expr: &Expr) -> Result<Value, Run_Err> {
        expr.accept(self)
    }
}
