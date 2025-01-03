use std::collections::{HashMap};

use crate::{
    ast::{
        expression::expr,
        statment::stmt::{self, Stmt},
    },
    interpret::interpreter::Interpreter,
};

use self::expr::Expr;

use X_Err;

use super::{Token};
#[derive(Clone)]
struct Resolver {
    inter: Box<Interpreter>,
    scopes: Vec<HashMap<String, bool>>,
}
impl Resolver {
    fn new(inter: Box<Interpreter>) -> Self {
        Self { inter,scopes: vec![] }
    }
    fn resolve_stmts(&self, stmts: &Vec<Stmt>)->Result<(), X_Err>  {
        for stmt in stmts {
            self.clone().resolve(stmt.clone())?;
        }
        Ok(())
    }
    fn beginScope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn decalre(&mut self,name:&Token){
          if !self.scopes.is_empty() {
            let scope=self.scopes.last().unwrap();
            self.scopes.push(HashMap::from([(name.lexeme.clone(),false)]));
          }
    }
    fn define(&mut self,name:&Token){
          if !self.scopes.is_empty() {
            self.scopes.last_mut().unwrap().insert(name.lexeme.clone(),true);
          }
    }
    fn endScope(&mut self) {
        self.scopes.pop();
    }
    fn resolve(&mut self, stmt: Stmt)->Result<(), X_Err> {
        stmt.accept(self)
    }
}
impl stmt::Visitor<Result<(), X_Err>> for Resolver {
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), X_Err> {
        self.beginScope();
        self.resolve_stmts(stmts);
        self.endScope();
        Ok(())
    }

    fn visit_break(&mut self) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_continue(&mut self) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_func(
        &mut self,
        name: &Option<Token>,
        func: &Box<Expr>,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &stmt::Stmt,
        else_branch: Option<&stmt::Stmt>,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_let(
        &mut self,
        name: &Token,
        expr: &Expr,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_return(
        &mut self,
        keyword: &Token,
        expr: &Expr,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_while(
        &mut self,
        condition: &Expr,
        body: &stmt::Stmt,
    ) -> Result<(), X_Err> {
        todo!()
    }
}
impl expr::Visitor<Result<(), X_Err>> for Resolver {
    fn visit_assign(&mut self, name: &Token, value: &Box<expr::Expr>) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_binary(
        &mut self,
        operator: &Token,
        l_expression: &expr::Expr,
        r_expression: &expr::Expr,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_call(
        &mut self,
        callee: &Box<expr::Expr>,
        paren: &Token,
        arguments: &Vec<Box<expr::Expr>>,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_func(
        &mut self,
        params: &Vec<Token>,
        body: &Vec<stmt::Stmt>,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_grouping(&mut self, expression: &expr::Expr) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_literal(&mut self, value: &crate::ast::token::object::Object) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_logical(
        &mut self,
        operator: &Token,
        l_expression: &Box<expr::Expr>,
        r_expression: &Box<expr::Expr>,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_ternary(
        &mut self,
        condition: &Box<expr::Expr>,
        t_expr: &Box<expr::Expr>,
        f_expr: &Box<expr::Expr>,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_unary(
        &mut self,
        operator: &Token,
        r_expression: &expr::Expr,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_variable(&mut self, name: &Token) -> Result<(), X_Err> {
        if(!self.scopes.is_empty()&&self.scopes.last().unwrap().get(&name.lexeme)==Some(&false)){
            return Err(X_Err::new(name.clone(),String::from("请不要在变量声明前使用变量")));
            
        }
        /**resolveLocal */
        return Ok(())
    }
}
trait Resolve<T>{
    fn resolve(&mut self, argu: T)->Result<(), X_Err>;
}
impl Resolve<Stmt> for Resolver {
    fn resolve(&mut self, stmt: Stmt)->Result<(), X_Err> {
        stmt.accept(self)
    }
}
impl Resolve<Expr> for Resolver {
    fn resolve(&mut self, expr: Expr)->Result<(), X_Err> {
        expr.accept(self)
    }
}