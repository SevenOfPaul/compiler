use std::collections::HashMap;
use crate::ast::expression::expr::Expr;
use crate::ast::token::token::Token;
use crate::interpret::value::Value;

#[derive(Clone, Debug,PartialEq, Eq,Hash)]
pub(crate) enum Stmt {
    Block {
        stmts: Vec<Stmt>,
    },
    Break {},
    Continue {},
    Expression {
        expr: Box<Expr>,
    },
    //函数声明
    Func {
        name: Option<Token>,
        func:Box<Expr>
    },
    IF {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    LET {
        name: Token,
        expr: Box<Expr>,
    },
    Print {
        expr: Box<Expr>,
    },
    Return {
        keyword: Token,
        expr: Box<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
    Impl{
        prototype:Token,
        methods:Vec<Stmt>
    }
}
pub trait Visitor<T> {
    fn visitor_block(&mut self, stmts: &Vec<Stmt>) -> T;
    fn visitor_break(&mut self) -> T;
    fn visitor_continue(&mut self) -> T;
    fn visitor_expr(&mut self, expr: &Expr) -> T;
    fn visitor_func(&mut self,stmt:&Stmt, name: &Option<Token>, func:&Box<Expr>) -> T;
    fn visitor_if(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: Option<&Stmt>) -> T;
    fn visitor_impl(&mut self,prototype:&Token,methods:&Vec<Stmt>)->T;
    fn visitor_let(&mut self, name: &Token, expr: &Expr) -> T;
    fn visitor_print(&mut self, expr: &Expr) -> T;
    fn visitor_return(&mut self, keyword: &Token, expr: &Expr) -> T;
    fn visitor_while(&mut self, condition: &Expr, body: &Stmt) -> T;
    // fn visitor_struct(&mut self,name:&Token,fields:&Vec<Token>)->T;
   
}
impl Stmt {
    pub(crate) fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Block { stmts } => visitor.visitor_block(stmts),
            Stmt::Break {} => visitor.visitor_break(),
            Stmt::Continue {} => visitor.visitor_continue(),
            Stmt::Expression { expr } => visitor.visitor_expr(expr),
            Stmt::IF {
                condition,
                then_branch,
                else_branch,
            } => {
                if else_branch.is_some() {
                    visitor.visitor_if(
                        condition,
                        then_branch,
                        Some(else_branch.clone().unwrap().as_ref()),
                    )
                } else {
                    visitor.visitor_if(condition, then_branch, None)
                }
            }
            Stmt::Func { name, func } =>visitor.visitor_func(self,name,func),
            Stmt::LET { name, expr } => visitor.visitor_let(name, expr),
            Stmt::Print { expr } => visitor.visitor_print(expr),
            Stmt::Return { keyword, expr } => visitor.visitor_return(keyword, expr),
            Stmt::While { condition, body } => visitor.visitor_while(condition, body),
            // Stmt::Struct { name, fields  } =>visitor.visitor_struct(name, fields ),
            Stmt::Impl { prototype, methods }=>visitor.visitor_impl(prototype,methods)
        }
    }
}
