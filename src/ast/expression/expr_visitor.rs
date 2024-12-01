use crate::interpret::env::enviroment;
use crate::ast::expression::expr::{Expr, Visitor};
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::env_get;

pub (crate) struct ExprVisitor {}
impl ExprVisitor{
    pub(crate) fn new() ->Self{
        ExprVisitor{} 
    }
    fn parenthesize(&mut self, operator:&str, exprs:Vec<&Expr>) ->String{
        let mut builder = String::from("(");
        builder.push_str(operator);
        for expr in exprs{
            builder.push_str(" ");
            builder.push_str(&expr.accept(self));
        }
        builder.push_str(")");
        builder
    }
}
impl Visitor<String> for ExprVisitor {
    fn visit_binary(&mut self, operator: &Token, l_expression: &Expr, r_expression: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(),vec![l_expression,r_expression])
    }

    fn visit_grouping(&mut self, expression: &Expr) -> String {
        self.parenthesize("group",vec![expression])
    }

    fn visit_literal(&mut self, value: &Object) -> String {
      value.to_string()
    }
    fn visit_unary(&mut self, operator: &Token, r_expression: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(),vec![r_expression])
    }
    fn visit_variable(&mut self, name: &Token) -> String {
      todo!();

    }
    fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> String {
        String::from("变量已声明")
    }
}