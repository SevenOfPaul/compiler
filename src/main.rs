use std::env;
use std::process::exit;
mod Run;
mod Error;
mod Lexer;
mod Token;
mod Scanner;

fn main() {
    let mut args = env::args();
    //获取命令行参数
    // println!("{}",args.nth(1).unwrap());
    if args.len()>2{
     Error::log(0,"X 是个脚本,参数传错啦");
        exit(64);
    }else if args.len()==2{
        //第一个是系统传入 第二个才是
        //Run file
        Run::run_file(args.nth(1).unwrap());
    }else{
        //Run prompt
        Error::log(0,"暂不支持命令行");
    }
}
