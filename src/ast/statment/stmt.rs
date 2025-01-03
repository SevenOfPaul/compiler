use crate::ast::expression::expr::Expr;
use crate::ast::token::token::Token;
#[derive(Clone, Debug)]
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
}
pub trait Visitor<T> {
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> T;
    fn visit_break(&mut self) -> T;
    fn visit_continue(&mut self) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_func(&mut self, name: &Option<Token>, func:&Box<Expr>) -> T;
    fn visit_if(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: Option<&Stmt>) -> T;
    fn visit_let(&mut self, name: &Token, expr: &Expr) -> T;
    fn visit_print(&mut self, expr: &Expr) -> T;
    fn visit_return(&mut self, keyword: &Token, expr: &Expr) -> T;
    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> T;
}
//  pub (crate) trait Accept<T>{
//         fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
//  }
impl Stmt {
    pub(crate) fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Block { stmts } => visitor.visit_block(stmts),
            Stmt::Break {} => visitor.visit_break(),
            Stmt::Continue {} => visitor.visit_continue(),
            Stmt::Expression { expr } => visitor.visit_expr(expr),
            Stmt::IF {
                condition,
                then_branch,
                else_branch,
            } => {
                if else_branch.is_some() {
                    visitor.visit_if(
                        condition,
                        then_branch,
                        Some(else_branch.clone().unwrap().as_ref()),
                    )
                } else {
                    visitor.visit_if(condition, then_branch, None)
                }
            }
            Stmt::Func { name, func } =>visitor.visit_func(name,func),
            Stmt::LET { name, expr } => visitor.visit_let(name, expr),
            Stmt::Print { expr } => visitor.visit_print(expr),
            Stmt::Return { keyword, expr } => visitor.visit_return(keyword, expr),
            Stmt::While { condition, body } => visitor.visit_while(condition, body),
        }
    }
}
