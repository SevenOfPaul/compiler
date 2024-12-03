use crate::ast::expression::expr::Expr;
use crate::ast::token::token::Token;
#[derive(Clone, Debug)]
pub(crate) enum Stmt {
    Block{stmts:Vec<Stmt>},
    Expression { expr: Box<Expr> },
    Print { expr: Box<Expr> },
    LET { name: Token, expr: Box<Expr> },
    IF{condition:Box<Expr>,then_branch:Box<Stmt>,else_branch:Option<Box<Stmt>>}
}
pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_print(&mut self, expr: &Expr) -> T;
    fn visit_let(&mut self, name: &Token, expr: &Expr) -> T;
    fn visit_block(&mut self, stmts:&Vec<Stmt>) -> T;
    fn visit_if(&mut self,condition:&Expr,then_branch:&Stmt,else_branch:Option<&Stmt>)->T;
}

impl Stmt {
    pub(crate) fn accept<T>(&self, visitor: &mut dyn Visitor<T>) {
        match self {
            Stmt::Expression { expr } => {
                visitor.visit_expr(expr.as_ref());
            }
            Stmt::Print { expr } => {
                
                visitor.visit_print(expr.as_ref());
            }
            Stmt::LET { name, expr } => {
                visitor.visit_let(name, expr.as_ref());
            }
            Stmt::Block { stmts } => {
                visitor.visit_block(stmts);
            }
            Stmt::IF { condition, then_branch, else_branch } => {
                if else_branch.is_some() {
                    visitor.visit_if(condition.as_ref(),then_branch.as_ref(),
                                     Some(else_branch.clone().unwrap().as_ref()));
                }else{
                    visitor.visit_if(condition.as_ref(),then_branch.as_ref(),None);
                }
            }
        }
    }
}
