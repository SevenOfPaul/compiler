extern crate core;

use std::env;
use std::process::exit;
mod ast;
mod error;
mod runner;
mod parse;
pub(crate) mod tools;
mod interpret;

use crate::runner::run::run_file;
fn main() {

    let mut args = env::args();
    run_file("let a=2;\
    print a;".parse().unwrap());
    //获取命令行参数
    // println!("{}",args.nth(1).unwrap());

    // if args.len() > 2 {
    //     error::log(0, "", "X 是个脚本,参数传错啦");
    //     exit(64);
    // } else if args.len() == 2 {
        //第一个是系统传入 第二个才是
        //runner file
        // run_file(args.nth(1).unwrap());
//     } else {
//         run_file("".parse().unwrap());
//         //runner prompt
//         error::log(0, "", "暂不支持命令行");
//     }
}
