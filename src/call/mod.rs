use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::interpret::value::Value;
use chrono::prelude::*;
lazy_static!{
    pub(crate) static ref Funcs: HashMap<String, (usize,Box<dyn Fn(Vec<Value>)->Value+Send+Sync+'static>)> = {
        //内置函数列表
        HashMap::from([
            (String::from("now"), 
               (0, Box::new(|arguments| {
                 let now = Local::now();
                     Value::Time(now)
                }) as Box<dyn Fn(Vec<Value>)->Value+Send+Sync+'static>)
            )
        ])
    };
}
pub(crate) trait Call{
    //检查参数数量
    fn arity(&self) -> usize;
    fn call(&self,arguments:Vec<Value>)->Value;
}// Box<dyn Fn(Vec<Value>) -> Value + Send + Sync + 'static>
#[derive(Debug, Clone)]
pub (crate) struct Func{
pub (crate) name:String
}
impl Func{
   pub(crate) fn new(name:&str)->Self{
           Self{name:String::from(name)}
    }
}
