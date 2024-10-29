extern crate core;

use std::env;
use std::process::exit;
mod ast;
mod error;
mod interpreter;
mod parse;
pub(crate) mod tools;

use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_type;
use ast::expr::*;
use tools::println;
fn main() {
    let expr = Expr::Binary {
        operator: Token::new(Token_type::MINUS, "-".to_owned(), None, 0),
        l_expression: Box::new(Expr::Literal {
            val: Some(Object::num(1.0)),
        }),
        r_expression: Box::new(Expr::Literal {
            val: Some(Object::num(2.0)),
        }),
    };
    let mut args = env::args();
    //获取命令行参数
    // println!("{}",args.nth(1).unwrap());

    println(&expr);
    if args.len() > 2 {
        error::log(0, 0, "X 是个脚本,参数传错啦");
        exit(64);
    } else if args.len() == 2 {
        //第一个是系统传入 第二个才是
        //interpreter file
        interpreter::run::run_file(args.nth(1).unwrap());
    } else {
        //interpreter prompt
        error::log(0, 0, "暂不支持命令行");
    }
}
