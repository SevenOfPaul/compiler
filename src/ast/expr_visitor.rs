use crate::ast::expr::{Expr, Visitor};
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_type;

pub (crate)struct ExprVisitor {}
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
    fn visit(&mut self, expr: &Expr) -> String {
     match expr {
         Expr::Binary {operator,l_expression,r_expression}=>{
               self.parenthesize(operator.lexeme.as_str(),vec![l_expression,r_expression])
         }
         Expr::Grouping {expression}=>{
             self.parenthesize("group",vec![expression])
         }
         Expr::Unary {operator,r_expression}=>{
             self.parenthesize(operator.lexeme.as_str(),vec![r_expression])
         }
         Expr::Literal {val}=>{
           if let Some(value)=val{
               value.to_string()
           }else{
              String::from("Nil")
           }
         }
     }
    }
}