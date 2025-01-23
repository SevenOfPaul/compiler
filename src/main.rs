use std::env;
use std::process::exit;
mod ast;
mod error;
mod runner;
mod parse;
mod tools;
mod interpret;
use crate::runner::run::{run_file, run_program};
fn main() {
    let mut args = env::args();
    //获取命令行参数
    if args.len() > 2 {
        error::log(0, "", "X 是个脚本,参数传错啦");
        exit(64);
    } else if args.len() == 2 {
        // 第一个是系统传入 第二个才是
        // runner file
        run_file(args.nth(1).unwrap());
    } else {
        run_program(String::new());
        error::log(0, "", "暂不支持命令行");
    }
}
