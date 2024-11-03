use std::fs::File;
use std::io::Read;
use std::process::exit;
use crate::error;
use crate::interpreter::scanner;
pub fn run_program(){
}
pub fn run_file(path: String) {
    //判断文件是否存在
    let mut bytes = String::new();
    if let Ok(mut res) = File::open(path) {
        res.read_to_string(&mut bytes).unwrap();
        let mut sc =scanner::Scanner::new(bytes);
        println!("{:?}", sc.scan_tokens());
    } else {
        error::log(0, "", "找不到文件");
        exit(32);
    }
}
