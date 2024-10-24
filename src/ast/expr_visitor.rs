use crate::ast::expr::{Expr, Visitor};

pub (crate)struct ExprVisitor {}
impl ExprVisitor{
    pub(crate) fn new() ->Self{
        ExprVisitor{} 
    }
}
impl Visitor<String> for ExprVisitor {
    fn visit(&self, expr: &Expr) -> String {
     match expr {
         Expr::Binary {operator,l_expression,r_expression}=>{

         }
         Expr::Grouping {expression}=>{

         }
         Expr::Unary {operator,r_expression}=>{}
         Expr::Literal {

         }
     }
    }
}