use crate::ast::scanner;
use crate::error;
use crate::interpret::env::resolve::Resolver;
use crate::interpret::interpreter::Interpreter;
use crate::parse::parser::Parser;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::error::X_Err;

fn run(bytes: String)->Result<(),X_Err> {
    let mut sc = scanner::Scanner::new(bytes);
    let mut parser = Parser::new(sc.scan_tokens());
    let stmts = Rc::new(parser.parse());
    let inter = Rc::from(RefCell::from(Interpreter::new()));
    let mut resolver = Resolver::new(inter.clone());
       resolver.resolve_stmts(&stmts)?;
    //调用解析出来的语句
    inter.borrow_mut().run(&stmts)?;
   Ok(())
}
#[wasm_bindgen]
pub fn run_program(bytes: String) {
    run(bytes);
}
pub fn run_file(path: String) {
    //判断文件是否存在
    // let mut bytes = String::from("for(let i=0;i<2;i=i+1){}");
    let mut bytes = String::new();
    // run(bytes);
    if let Ok(mut res) = File::open(path) {
        let _=res.read_to_string(&mut bytes);
        run(bytes);
    } else {
        error::log(0, "", "找不到文件");
        exit(32);
    }
}
