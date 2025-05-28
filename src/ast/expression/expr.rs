use crate::ast::statment::stmt::Stmt;
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::impl_expr_accept;
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Expr {
    //声明变量
    Assign {
        name: Token,
        val: Box<Expr>,
    },
    Binary {
        operator: Token,
        l_expression: Box<Expr>,
        r_expression: Box<Expr>,
    },
    //调用函数
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    Func {
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Instance {
        struct_name: Box<Expr>,
        keys: Vec<Token>,
        vals: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        val: Object,
    },
    //逻辑运算符
    Logical {
        operator: Token,
        l_expression: Box<Expr>,
        r_expression: Box<Expr>,
    },
    Struct {
        name: Token,
        ///这个是struct的静态数据
        fields: Vec<Token>,
    },
    Set{
        object: Box<Expr>,
        name: Token,
        val: Box<Expr>, 
    },
    //三元
    Ternary {
        condition: Box<Expr>,
        t_expr: Box<Expr>,
        f_expr: Box<Expr>,
    },
    Unary {
        operator: Token,
        r_expression: Box<Expr>,
    },
    //获取变量
    Variable {
        name: Token,
    },
}
//  pub (crate) trait Accept<T>{
//         fn accept(&self, visitor: &mut dyn visitor<T>) -> T;
//  }
pub trait Visitor<T> {
    fn visitor_assign(&mut self, expr: &Expr, name: &Token, value: &Box<Expr>) -> T;
    fn visitor_binary(&mut self, operator: &Token, l_expression: &Expr, r_expression: &Expr) -> T;
    fn visitor_call(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Box<Expr>>) -> T;
    fn visitor_func(&mut self, params: &Vec<Token>, body: &Vec<Stmt>) -> T;
    fn visitor_grouping(&mut self, expression: &Expr) -> T;
    fn visitor_get(&mut self, object: &Expr, name: &Token) -> T;
    fn visitor_literal(&mut self, value: &Object) -> T;
    fn visitor_logical(
        &mut self,
        operator: &Token,
        l_expression: &Box<Expr>,
        r_expression: &Box<Expr>,
    ) -> T;
    fn visitor_ternary(&mut self, condition: &Box<Expr>, t_expr: &Box<Expr>, f_expr: &Box<Expr>)
        -> T;
    fn visitor_set(&mut self, object: &Expr, name: &Token, val: &Box<Expr>) -> T;
    fn visitor_unary(&mut self, operator: &Token, r_expression: &Expr) -> T;
    fn visitor_variable(&mut self, expr: &Expr, name: &Token) -> T;
    fn visitor_instance(
        &mut self,
        struct_name: &Box<Expr>,
        keys: &Vec<Token>,
        vals: &Vec<Expr>,
    ) -> T;
     fn visitor_struct(
        &mut self,
         name: &Token,
       fields: &Vec<Token>,
    ) -> T;
}
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Literal { val } => visitor.visitor_literal(val),
            Expr::Func { params, body } => visitor.visitor_func(params, body),
            Expr::Grouping { expression } => visitor.visitor_grouping(expression),
            Expr::Binary {
                operator,
                l_expression,
                r_expression,
            } => visitor.visitor_binary(operator, l_expression, r_expression),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visitor_call(callee, paren, arguments),
            Expr::Unary {
                operator,
                r_expression,
            } => visitor.visitor_unary(operator, r_expression),
            Expr::Variable { name } => visitor.visitor_variable(self, name),
            Expr::Instance {
                struct_name,
                keys,
                vals,
            } => visitor.visitor_instance(struct_name, keys, vals),
            Expr::Assign { name, val } => visitor.visitor_assign(self, name, val),
            Expr::Ternary {
                condition,
                t_expr,
                f_expr,
            } => visitor.visitor_ternary(condition, t_expr, f_expr),
            Expr::Struct { name, fields } => visitor.visitor_struct(name, fields),
            Expr::Set { object, name, val }=> visitor.visitor_set(object, name,val),
            Expr::Logical {
                operator,
                l_expression,
                r_expression,
            } => visitor.visitor_logical(operator, l_expression, r_expression),
            Expr::Get { object, name } => visitor.visitor_get(object, name),
        }
    }
}

// impl_expr_accept! {
//     (Literal,literal,{val,}),(Func,func,{params,body,}),
//     (Grouping,grouping,{expression,}),(Binary,binary,{operator,l_expression,r_expression,}),
//     (Call,call,{callee,paren,arguments,}),
//     (Unary,unary,{operator,r_expression,}
// ),(Variable,variable,{expr,name,}),(
//     Assign,assign,{name,val,}
// ),(Ternary,ternary,{condition,t_expr,f_expr,}),(
//     Logical,logical,{operator,l_expression,r_expression,}
// ),}
