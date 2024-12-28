use crate::interpret::value::Value;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
pub(crate) fn println(v:Value){
    match v {
        Value::Num(num) => printf( num),
        Value::Str(s)=>printf( s),
        Value::Bool(b)=>printf( b),
        Value::Time(t)=>printf(t),
         Value::Func(f)=>printf(f.to_string()),
        _=>printf(Value::Nil)
    }
}


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn append_to_output(s: &str, is_error: bool);
}

#[cfg(not(target_arch = "wasm32"))]
fn append_to_output(s: &str, is_error: bool) {
    println!("{}", s);
}

pub fn printf<T: std::fmt::Display>(val: T) {
    append_to_output(&format!("{}", val), false);
}