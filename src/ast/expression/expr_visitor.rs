use crate::ast::expression::expr::{Expr};
use crate::ast::statment::stmt::Stmt;
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;

use super::expr::Visitor;

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
    fn visitor_binary(&mut self, operator: &Token, l_expression: &Expr, r_expression: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(),vec![l_expression,r_expression])
    }

    fn visitor_grouping(&mut self, expression: &Expr) -> String {
        self.parenthesize("group",vec![expression])
    }

    fn visitor_literal(&mut self, value: &Object) -> String {
      value.to_string()
    }
    fn visitor_unary(&mut self, operator: &Token, r_expression: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(),vec![r_expression])
    }
    fn visitor_variable(&mut self,expr: &Expr, _name: &Token) -> String {
      todo!();

    }
    fn visitor_assign(&mut self,_:&Expr, _name: &Token,_value: &Box<Expr>) -> String {
        String::from("变量已声明")
    }

    fn visitor_ternary(&mut self, _condition: &Box<Expr>, _t_expr: &Box<Expr>, _f_expr: &Box<Expr>) -> String {
        todo!()
    }

    fn visitor_logical(&mut self, _operator: &Token, _l_expression: &Box<Expr>, _r_expression: &Box<Expr>) -> String {
        todo!()
    }

    fn visitor_call(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Box<Expr>>) -> String {
        todo!()
    }
    
    fn visitor_func(&mut self,params:&Vec<Token>,body:&Vec<crate::ast::statment::stmt::Stmt>)->String {
        todo!()
    }
    
    fn visitor_get(&mut self,object:&Expr,name:&Token)->String {
        todo!()
    }
    
    fn visitor_instance(&mut self,struct_name:&Box<Expr>, keys: &Vec<Token>,vals:&Vec<Expr>) -> String {
        todo!()
    }
    
    fn visitor_struct(
        &mut self,
         name: &Token,
       fields: &Vec<Token>,
    ) -> String {
        todo!()
    }
    
    fn visitor_set(&mut self, object: &Expr, name: &Token, val: &Box<Expr>) -> String {
        todo!()
    }  

}