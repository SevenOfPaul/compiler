use std::{cell::RefCell, collections::HashMap, rc::Rc};
use std::cmp::PartialEq;
use crate::{
    ast::{
        expression::expr::Visitor,
        statment::stmt::{self, Stmt},
    },
    interpret::interpreter::Interpreter,
};

use crate::ast::expression::expr::Expr;

use super::{Run_Err, Token, X_Err};
/**只能有一个 */
pub(crate) struct Resolver {
    inter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    cur_fn:Rc<FN_TYPE>
}
impl Resolver {
    pub(crate) fn new(inter: Rc<RefCell<Interpreter>>) -> Self {
        Self {
            inter,
            scopes: vec![],
            cur_fn:Rc::from(FN_TYPE::None)
        }
    }
    pub(crate) fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<(), X_Err> {
        for stmt in stmts {
            self.resolve(stmt.clone())?;
        }
        Ok(())
    }
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn declare(&mut self, name: &Token)->Result<(),X_Err> {
        if !self.scopes.is_empty() {
           let scope=self.scopes.last_mut().unwrap();
            if scope.contains_key(&name.lexeme) {
                return Err(Run_Err::new(name.clone(), String::from("请不要重复声明变量")))
            }else{
                scope.insert(name.lexeme.clone(), false);
            }
        }
        Ok(())
    }
    fn define(&mut self, name: &Token) {
        if !self.scopes.is_empty() {
            self.scopes
                .last_mut()
                .unwrap()
                .insert(name.lexeme.clone(), true);
        }
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
    }
    fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<(), X_Err> {
        for (idx, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                //调用解释器的resolve函数
                self.inter
                    .borrow_mut()
                    .resolve(expr, (self.scopes.len()-1  - idx) as i32)?;
                return Ok(());
            }
        }
        Ok(())
    }
}

impl stmt::Visitor<Result<(), X_Err>> for Resolver {
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), X_Err> {
        self.begin_scope();
        self.resolve_stmts(stmts)?;
        self.end_scope();
        Ok(())
    }

    fn visit_break(&mut self) -> Result<(), X_Err> {
        Ok(())
    }

    fn visit_continue(&mut self) -> Result<(), X_Err> {
        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<(), X_Err> {
        self.resolve(expr.clone())?;
        Ok(())
    }

    fn visit_func(
        &mut self,
        stmt: &Stmt,
        name: &Option<Token>,
        func: &Box<Expr>,
    ) -> Result<(), X_Err> {
        if let Some(n) = name {
            self.declare(&name.clone().unwrap())?;
            self.define(&name.clone().unwrap());
        }
        self.resolve_func(stmt,Rc::from(FN_TYPE::FN))?;
        Ok(())
    }
    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &stmt::Stmt,
        else_branch: Option<&stmt::Stmt>,
    ) -> Result<(), X_Err> {
        self.resolve(condition.clone())?;
        self.resolve(then_branch.clone())?;
        if let Some(branch) = else_branch {
            self.resolve(branch.clone())?;
        }
        Ok(())
    }

    fn visit_let(&mut self, name: &Token, expr: &Expr) -> Result<(), X_Err> {
        self.declare(name);
        self.resolve(expr.clone())?;
        self.define(name);
        Ok(())
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<(), X_Err> {
        self.resolve(expr.clone())?;
        Ok(())
    }

    fn visit_return(&mut self, keyword: &Token, expr: &Expr) -> Result<(), X_Err> {
        self.resolve(expr.clone())?;
        if *(self.cur_fn.as_ref()) == FN_TYPE::None {
          return Err(Run_Err::new(keyword.clone(),String::from("请不要在函数外使用return")));
        }
        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, body: &stmt::Stmt) -> Result<(), X_Err> {
        self.resolve(condition.clone())?;
        self.resolve(body.clone())?;
        Ok(())
    }

    fn visit_struct(&mut self, name: &Token, methods: &Vec<Stmt>,fields:&Vec<Token>) -> Result<(), X_Err> {
        self.declare(name)?;
        self.define(name);
        Ok(())
    }
    
    fn visit_impl(&mut self,prototype:&Token,methods:&Vec<Stmt>)->Result<(), X_Err> {
        //需要修改
        todo!()
    }
    
}
impl Visitor<Result<(), X_Err>> for Resolver {
    fn visit_assign(&mut self, expr: &Expr, name: &Token, value: &Box<Expr>) -> Result<(), X_Err> {
        self.resolve(value.as_ref().clone())?;
        //这里得改
        self.resolve_local(expr, name)?;
        Ok(())
    }

    fn visit_binary(
        &mut self,
        operator: &Token,
        l_expression: &Expr,
        r_expression: &Expr,
    ) -> Result<(), X_Err> {
        self.resolve(l_expression.clone())?;
        self.resolve(r_expression.clone())?;
        Ok(())
    }

    fn visit_call(
        &mut self,
        callee: &Box<Expr>,
        paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> Result<(), X_Err> {
        self.resolve(callee.as_ref().clone())?;
        for arg in arguments {
            self.resolve(arg.as_ref().clone())?;
        }
        Ok(())
    }

    fn visit_func(&mut self, params: &Vec<Token>, body: &Vec<stmt::Stmt>) -> Result<(), X_Err> {
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        for stmt in body {
            self.resolve(stmt.clone())?;
        }
        Ok(())
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Result<(), X_Err> {
        self.resolve(expression.clone())?;
        Ok(())
    }

    fn visit_literal(&mut self, value: &crate::ast::token::object::Object) -> Result<(), X_Err> {
        //没有表达式 是个值
        Ok(())
    }

    fn visit_logical(
        &mut self,
        operator: &Token,
        l_expression: &Box<Expr>,
        r_expression: &Box<Expr>,
    ) -> Result<(), X_Err> {
        self.resolve(l_expression.as_ref().clone())?;
        self.resolve(r_expression.as_ref().clone())?;
        Ok(())
    }

    fn visit_ternary(
        &mut self,
        condition: &Box<Expr>,
        t_expr: &Box<Expr>,
        f_expr: &Box<Expr>,
    ) -> Result<(), X_Err> {
        self.resolve(condition.as_ref().clone())?;
        self.resolve(t_expr.as_ref().clone())?;
        self.resolve(f_expr.as_ref().clone())?;
        Ok(())
    }

    fn visit_unary(&mut self, operator: &Token, r_expression: &Expr) -> Result<(), X_Err> {
        self.resolve(r_expression.clone())?;
        Ok(())
    }

    fn visit_variable(&mut self, expr: &Expr, name: &Token) -> Result<(), X_Err> {
        if !self.scopes.is_empty() {
            if let Some(&initialized) = self.scopes.last().unwrap().get(&name.lexeme) {
                if !initialized {
                    return Err(Run_Err::new(
                        name.clone(),
                        String::from("变量无法在声明前使用"),
                    ));
                }
            }
        }
        self.resolve_local(expr, name)?;
        Ok(())
    }
    
    fn visit_get(&mut self,object:&Expr,name:&Token)->Result<(), X_Err> {
        self.resolve(object.clone())?;
        Ok(())
    }
}
trait Resolve<T> {
    fn resolve(&mut self, argu: T) -> Result<(), X_Err>;
    fn resolve_func(&mut self, argu: &T,typ:Rc<FN_TYPE>) -> Result<(), X_Err>;
}
impl Resolve<Stmt> for Resolver {
    fn resolve(&mut self, stmt: Stmt) -> Result<(), X_Err> {
        stmt.accept(self)
    }
    fn resolve_func(&mut self, stmt: &Stmt,typ:Rc<FN_TYPE>) -> Result<(), X_Err> {
        let enclose_typ=self.cur_fn.clone();
        self.cur_fn=typ.clone();
        match stmt {
            Stmt::Func { name, func } => {
                //这里有问题
                self.begin_scope();
                self.resolve_func(func.as_ref(),typ)?;
                self.end_scope();
            }
            _ => {}
        }
        self.cur_fn=enclose_typ;
        Ok(())
    }
}
impl Resolve<Expr> for Resolver {
    fn resolve(&mut self, expr: Expr) -> Result<(), X_Err> {
        expr.accept(self)
    }
    fn resolve_func(&mut self, expr: &Expr,_:Rc<FN_TYPE>) -> Result<(), X_Err> {
        match expr {
            Expr::Func { params, body } => {
                self.visit_func(params, body)?;
            }
            _ => {}
        }
        Ok(())
    }
}
#[derive(Debug)]
enum FN_TYPE {
FN ,
None
}
impl PartialEq for FN_TYPE {
    fn eq(&self, other: &Self) -> bool {
       match self {
           FN_TYPE::FN=>{
               if let FN_TYPE::FN=other {
                   return true
               }
           },
           _=>{
               if let FN_TYPE::None=other {
                  return true
               }
           }
       }
        false
    }
}
