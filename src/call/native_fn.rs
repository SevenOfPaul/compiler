//自带的fn
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
            ),    (String::from("X"), 
               (0, Box::new(|arguments| {
                    println!("{:?}","==================================");
                    println!("{:?}","==  X-MAN   ======================");
                    println!("{:?}","==================================");
                    Value::Nil
                    
                }) as Box<dyn Fn(Vec<Value>)->Value+Send+Sync+'static>)
            )
        ])
    };
}
#[derive(Debug, Clone)]
pub (crate) struct Func{
pub (crate) name:String
}
impl Func{
    //这是个媒介，实际发生函数存储在Funcs中
   pub(crate) fn new(name:&str)->Self{
           Self{name:String::from(name)}
    }
    pub (crate) fn to_string(&self)->String{
         String::from("<native fn>")
    }
}