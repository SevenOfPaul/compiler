use std::env;
use std::process::exit;
mod error;
mod interpreter;
fn main() {
    let mut args = env::args();
    //获取命令行参数
    // println!("{}",args.nth(1).unwrap());
    if args.len()>2{
     error::log(0, 0, "X 是个脚本,参数传错啦");
        exit(64);
    }else if args.len()==2{
        //第一个是系统传入 第二个才是
        //interpreter file
        interpreter::run_file(args.nth(1).unwrap());
    }else{
        //interpreter prompt
        error::log(0, 0, "暂不支持命令行");
    }
}
