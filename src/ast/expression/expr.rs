use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::impl_expr_accept;
use paste::paste;
use std::fmt::Binary;
#[derive(Debug,Clone)]
pub(crate) enum Expr {
    Binary {
        operator: Token,
        l_expression: Box<Expr>,
        r_expression: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        val: Object,
    },
    Unary {
        operator: Token,
        r_expression: Box<Expr>,
    },
    Variable {
        name:Token
    },
    Assign{
        name:Token,
        val:Box<Expr>,
    }
    Question{
        condition:&Expr,
        then_expr:Box<Expr>,
        else_expr:Box<Expr>
    }
}

pub trait Visitor<T> {
    fn visit_binary(&mut self, operator: &Token, l_expression: &Expr, r_expression: &Expr) -> T;
    fn visit_grouping(&mut self, expression: &Expr) -> T;
    fn visit_literal(&mut self, value: &Object) -> T;
    fn visit_unary(&mut self, operator: &Token, r_expression: &Expr) -> T;
    fn visit_variable(&mut self, name: &Token) -> T;
    fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> T;
    fn visit_question(&mut self,condition:&Expr,then_expr:&Box<Expr>,else_expr:&Box<Expr>)->T
}
// 
// impl Expr {
//     pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
//       match  self {
//          Expr::Literal {val}=>{
//              visitor.visit_literal(val)
//          }
//           Expr::Grouping {expression}=>{
//               visitor.visit_grouping(expression)
//           }
//           Expr::Binary {operator, l_expression, r_expression}=>{
//               visitor.visit_binary(operator,l_expression,r_expression)
//           }
//           Expr::Unary {operator, r_expression}=>{
//               visitor.visit_unary(operator,r_expression)
//           },
//           Expr::Variable { name } =>{
//               visitor.visit_variable(name)
//           }
//       }
//     }
// }
impl_expr_accept! {(Literal,literal,{val,}),(
    Grouping,grouping,{expression,}
),(Binary,binary,{operator,l_expression,r_expression,}),(
    Unary,unary,{operator,r_expression,}
),(Variable,variable,{name,}),(
    Assign,assign,{name,val,}
),(
    Question,question,{
        condition,then_expr,else_expr  
    }
)}
