use std::collections::{HashMap};

use crate::{
    ast::{
        expression::expr,
        statment::stmt::{self, Stmt},
    },
    interpret::interpreter::Interpreter,
};

use self::expr::Expr;

use super::{Run_Err, Token, X_Err};

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
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn decalre(&mut self,name:&Token){
          if !self.scopes.is_empty() {
            let _=self.scopes.last().unwrap();
            self.scopes.push(HashMap::from([(name.lexeme.clone(),false)]));
          }
    }
    fn define(&mut self,name:&Token){
          if !self.scopes.is_empty() {
            self.scopes.last_mut().unwrap().insert(name.lexeme.clone(),true);
          }
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
    }
    fn resolve_local(&mut self,expr:&Expr,name:&Token)->Result<(),X_Err>{
        for (idx,scope) in self.scopes.iter().enumerate().rev(){
            if scope.contains_key(&name.lexeme) {
                //调用解释器的resolve函数
                return Ok(());
            }
        }
        Ok(())
    }
}
impl stmt::Visitor<Result<(), X_Err>> for Resolver {
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), X_Err> {
        self.begin_scope();
        self.resolve_stmts(stmts);
        self.end_scope();
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
   if let Some(n)=name{
         self.decalre(&name.clone().unwrap());
        self.define(&name.clone().unwrap());
   }
   self.resolve_func(&Stmt::Func { name: name.clone(), func: func.clone()})?;
   Ok(())
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
      self.decalre(name);
    self.resolve(expr.clone())?;
      self.define(name);
      Ok(())
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
        self.resolve(value.as_ref().clone())?;
        self.resolve_local(&Expr::Assign { name: name.clone(), val: value.clone()},name);
        Ok(())
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
        if !self.scopes.is_empty()&&self.scopes.last().unwrap().get(&name.lexeme)==Some(&false){
            return Err(Run_Err::new(name.clone(),String::from("请不要在变量声明前使用变量")));

        }
        self.resolve_local(&Expr::Variable{name:name.clone()},name);
         Ok(())
    }
}
trait Resolve<T>{
    fn resolve(&mut self, argu: T)->Result<(), X_Err>;
    fn resolve_func(&mut self,argu:&T)->Result<(),X_Err>;
}
impl Resolve<Stmt> for Resolver {
    fn resolve(&mut self, stmt: Stmt)->Result<(), X_Err> {
        stmt.accept(self)
    }
    fn resolve_func(&mut self, stmt: &Stmt)->Result<(), X_Err> {
      match stmt {
          Stmt::Func{name,func}=>self.resolve_func(func.as_ref())?,
          _=>{}
      }
      Ok(())
    }
}
impl Resolve<Expr> for Resolver {
    fn resolve(&mut self, expr: Expr)->Result<(), X_Err> {
        expr.accept(self)
    }
    fn resolve_func(&mut self,expr:&Expr)->Result<(),X_Err>{
        todo!()
    }
}