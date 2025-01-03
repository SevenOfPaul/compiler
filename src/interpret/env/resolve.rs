use crate::{
    ast::{
        expression::expr,
        statment::stmt::{self, Stmt},
    },
    interpret::interpreter::Interpreter,
};

use super::X_Err;
#[derive(Clone)]
struct Resolver {
    inter: Box<Interpreter>,
}
impl Resolver {
    fn new(inter: Box<Interpreter>) -> Self {
        Self { inter }
    }
    fn resolve_stmts(&self, stmts: &Vec<Stmt>)->Result<(), X_Err>  {
        for stmt in stmts {
            self.clone().resolve_stmt(stmt.clone())?;
        }
        Ok(())
    }
    fn resolve_stmt(&mut self, stmt: Stmt)->Result<(), X_Err> {
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

    fn visit_expr(&mut self, expr: &crate::ast::expression::expr::Expr) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_func(
        &mut self,
        name: &Option<super::Token>,
        func: &Box<crate::ast::expression::expr::Expr>,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_if(
        &mut self,
        condition: &crate::ast::expression::expr::Expr,
        then_branch: &stmt::Stmt,
        else_branch: Option<&stmt::Stmt>,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_let(
        &mut self,
        name: &super::Token,
        expr: &crate::ast::expression::expr::Expr,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_print(&mut self, expr: &crate::ast::expression::expr::Expr) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_return(
        &mut self,
        keyword: &super::Token,
        expr: &crate::ast::expression::expr::Expr,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_while(
        &mut self,
        condition: &crate::ast::expression::expr::Expr,
        body: &stmt::Stmt,
    ) -> Result<(), X_Err> {
        todo!()
    }
}
impl expr::Visitor<Result<(), X_Err>> for Resolver {
    fn visit_assign(&mut self, name: &super::Token, value: &Box<expr::Expr>) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_binary(
        &mut self,
        operator: &super::Token,
        l_expression: &expr::Expr,
        r_expression: &expr::Expr,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_call(
        &mut self,
        callee: &Box<expr::Expr>,
        paren: &super::Token,
        arguments: &Vec<Box<expr::Expr>>,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_func(
        &mut self,
        params: &Vec<super::Token>,
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
        operator: &super::Token,
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
        operator: &super::Token,
        r_expression: &expr::Expr,
    ) -> Result<(), X_Err> {
        todo!()
    }

    fn visit_variable(&mut self, name: &super::Token) -> Result<(), X_Err> {
        todo!()
    }
}
