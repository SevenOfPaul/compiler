use crate::ast::statment::stmt::Stmt;
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::impl_expr_accept;
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
    Get{
        object:Box<Expr>,
        name:Token
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
//         fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
//  }
pub trait Visitor<T> {
    fn visit_assign(&mut self,expr:&Expr, name: &Token, value: &Box<Expr>) -> T;
    fn visit_binary(&mut self, operator: &Token, l_expression: &Expr, r_expression: &Expr) -> T;
    fn visit_call(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Box<Expr>>) -> T;
    fn visit_func(&mut self, params: &Vec<Token>, body: &Vec<Stmt>) -> T;
    fn visit_grouping(&mut self, expression: &Expr) -> T;
    fn visit_get(&mut self,object:&Expr,name:&Token)->T;
    fn visit_literal(&mut self, value: &Object) -> T;
    fn visit_logical(
        &mut self,
        operator: &Token,
        l_expression: &Box<Expr>,
        r_expression: &Box<Expr>,
    ) -> T;
    fn visit_ternary(&mut self, condition: &Box<Expr>, t_expr: &Box<Expr>, f_expr: &Box<Expr>)
        -> T;
    fn visit_unary(&mut self, operator: &Token, r_expression: &Expr) -> T;
    fn visit_variable(&mut self, expr: &Expr, name: &Token) -> T;
}
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Literal { val } => visitor.visit_literal(val),
            Expr::Func { params, body } => visitor.visit_func(params, body),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Binary {
                operator,
                l_expression,
                r_expression,
            } => visitor.visit_binary(operator, l_expression, r_expression),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call(callee, paren, arguments),
            Expr::Unary {
                operator,
                r_expression,
            } => visitor.visit_unary(operator, r_expression),
            Expr::Variable {  name } => visitor.visit_variable(self, name),
            Expr::Assign { name, val } => visitor.visit_assign(self,name, val),
            Expr::Ternary {
                condition,
                t_expr,
                f_expr,
            } => visitor.visit_ternary(condition, t_expr, f_expr),
            Expr::Logical {
                operator,
                l_expression,
                r_expression,
            } => visitor.visit_logical(operator, l_expression, r_expression),
            Expr::Get { object, name } => visitor.visit_get(object,name),
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
