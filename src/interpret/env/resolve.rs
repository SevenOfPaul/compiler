use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::interpret::env::fn_type::FN_TYPE;
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
            //当前函数栈
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
    fn visitor_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), X_Err> {
        self.begin_scope();
        self.resolve_stmts(stmts)?;
        self.end_scope();
        Ok(())
    }

    fn visitor_break(&mut self) -> Result<(), X_Err> {
        Ok(())
    }

    fn visitor_continue(&mut self) -> Result<(), X_Err> {
        Ok(())
    }

    fn visitor_expr(&mut self, expr: &Expr) -> Result<(), X_Err> {
        self.resolve(expr.clone())?;
        Ok(())
    }

    fn visitor_func(
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
    fn visitor_if(
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

    fn visitor_let(&mut self, name: &Token, expr: &Expr) -> Result<(), X_Err> {
        let _=self.declare(name);
        self.resolve(expr.clone())?;
        self.define(name);
        Ok(())
    }

    fn visitor_print(&mut self, expr: &Expr) -> Result<(), X_Err> {
        self.resolve(expr.clone())?;
        Ok(())
    }

    fn visitor_return(&mut self, keyword: &Token, expr: &Expr) -> Result<(), X_Err> {
        self.resolve(expr.clone())?;
        if *(self.cur_fn.as_ref()) == FN_TYPE::None {
          return Err(Run_Err::new(keyword.clone(),String::from("请不要在函数外使用return")));
        }
        Ok(())
    }

    fn visitor_while(&mut self, condition: &Expr, body: &stmt::Stmt) -> Result<(), X_Err> {
        self.resolve(condition.clone())?;
        self.resolve(body.clone())?;
        Ok(())
    }

    
    fn visitor_impl(&mut self,prototype:&Token,methods:&Vec<Stmt>)->Result<(), X_Err> {
        //需要修改
        todo!()
    }
    
}
impl Visitor<Result<(), X_Err>> for Resolver {
    fn visitor_assign(&mut self, expr: &Expr, name: &Token, value: &Box<Expr>) -> Result<(), X_Err> {
        self.resolve(value.as_ref().clone())?;
        //这里得改
        self.resolve_local(expr, name)?;
        Ok(())
    }

    fn visitor_binary(
        &mut self,
        operator: &Token,
        l_expression: &Expr,
        r_expression: &Expr,
    ) -> Result<(), X_Err> {
        self.resolve(l_expression.clone())?;
        self.resolve(r_expression.clone())?;
        Ok(())
    }

    fn visitor_call(
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

    fn visitor_func(&mut self, params: &Vec<Token>, body: &Vec<stmt::Stmt>) -> Result<(), X_Err> {

        //需要在这里定义函数栈
        self.resolve_func(&Expr::Func { params:params.clone(), body:body.clone() },Rc::from(FN_TYPE::FN))?;

        Ok(())
    }
    fn visitor_instance(&mut self, struct_name: &Box<Expr>, keys: &Vec<Token>, vals: &Vec<Expr>) -> Result<(), X_Err> {
       self.resolve(struct_name.as_ref().clone())?;
        for val in vals{
            self.resolve(val.clone())?;
        }
       Ok(())
    }
    fn visitor_grouping(&mut self, expression: &Expr) -> Result<(), X_Err> {
        self.resolve(expression.clone())?;
        Ok(())
    }

    fn visitor_literal(&mut self, value: &crate::ast::token::object::Object) -> Result<(), X_Err> {
        //没有表达式 是个值
        Ok(())
    }

    fn visitor_logical(
        &mut self,
        operator: &Token,
        l_expression: &Box<Expr>,
        r_expression: &Box<Expr>,
    ) -> Result<(), X_Err> {
        self.resolve(l_expression.as_ref().clone())?;
        self.resolve(r_expression.as_ref().clone())?;
        Ok(())
    }
    fn visitor_struct(&mut self, name: &Token,fields:&Vec<Token>) -> Result<(), X_Err> {
        self.declare(name)?;
        self.define(name);
        Ok(())
    }
    fn visitor_ternary(
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

    fn visitor_unary(&mut self, operator: &Token, r_expression: &Expr) -> Result<(), X_Err> {
        self.resolve(r_expression.clone())?;
        Ok(())
    }

    fn visitor_variable(&mut self, expr: &Expr, name: &Token) -> Result<(), X_Err> {
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
    
    fn visitor_get(&mut self,object:&Expr,name:&Token)->Result<(), X_Err> {
        self.resolve(object.clone())?;
        Ok(())
    }
    fn visitor_set(&mut self, object: &Expr, name: &Token, val: &Box<Expr>) -> Result<(), X_Err> {
        self.resolve(val.as_ref().clone())?;
        self.resolve(object.clone())?;
        //这里需要修改
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
    fn resolve_func(&mut self, expr: &Expr,typ:Rc<FN_TYPE>) -> Result<(), X_Err> {
        let enclose_typ=self.cur_fn.clone();
        self.cur_fn=typ.clone();
        //开启局部作用域
        self.begin_scope();
        match expr {
            Expr::Func { params, body } => {
                for param in params {
                    self.declare(param)?;
                    self.define(param);
                }
                for stmt in body {
                    self.resolve(stmt.clone())?;
                }
            }
            _ => {}
        }
        self.end_scope();
        self.cur_fn=enclose_typ;
        Ok(())
    }
}

