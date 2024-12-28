#[cfg(target_arch = "wasm32")]
pub fn log(line: usize, pos: &str, message: &str) {
    use crate::tools::printf;
    printf(format!("错误: 在行{}位置{}发生了'{}'这个错误！", line, pos, message));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(line: usize, pos: &str, message: &str) {
    use color_string::Font::*;
    use color_string::{cs, fonts};
    let fonts = fonts!(Red);
    println!("{}", cs!(fonts=>"请检查你的代码"));
    println!("{}", cs!(fonts=>format!("错误:在行{}位置{}发生了'{}'这个错误！ ",line,pos,message)))
}
