use crate::ast::expr::{Expr, Visitor};
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type::NIL;
use crate::interpret::error::Run_Err;
use crate::interpret::value::Value;

struct Interpreter {}
impl Visitor<Result<Value,Run_Err>> for Interpreter {
    fn visit_binary(&mut self, operator: &Token, l_expression: &Expr, r_expression: &Expr) -> Result<Value, Run_Err> {
        todo!()
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Result<Value, Run_Err> {
        todo!()
    }

    fn visit_literal(&mut self, value: &Object) -> Result<Value, Run_Err> {
        todo!()
    }
    fn visit_unary(&mut self, operator: &Token, r_expression: &Expr) -> Result<Value, Run_Err> {
        todo!()
    }
}
impl Interpreter {
    fn visit_literal(&self,expr: &Expr){
        // expr.val
    }
}