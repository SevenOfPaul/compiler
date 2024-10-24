use color_string::Font::*;
use color_string::{cs, fonts, pcs, wcs, wf, Colored, FontTool};
pub fn log(line :usize,pos:usize, message:&str){
 report(line,pos,message);
}
fn report(line :usize,pos:usize,message:&str){
    let fonts = fonts!(Red);
   println!("{}",cs!(fonts =>format!("错误:在行{}位置{}发生了'{}'这个错误！ ",line,pos,message)));
}
