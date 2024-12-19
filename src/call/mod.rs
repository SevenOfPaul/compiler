use crate::interpret::{interpreter::Interpreter, value::Value};
mod native_fn;
mod decl_fn;
use native_fn::Native_Fn;
use decl_fn::Decl_Fn;
/**语言内部自带的函数 */
use std::collections::HashMap;
use lazy_static::lazy_static;
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
pub (crate) enum Func{
    Native(Native_Fn),
    DeclFn(Decl_Fn)
}
pub(crate) trait Call{
    //检查参数数量
    fn arity(&self) -> usize;
    fn call(&self, inter: &mut Interpreter,arguments:Vec<Value>)->Value;

}// Box<dyn Fn(Vec<Value>) -> Value + Send + Sync + 'static>
impl Call for Func{
    fn arity(&self) -> usize {
      match self{
          Func::Native(n_f)=>n_f.arity(),
          Func::DeclFn(d_f)=>d_f.arity()
    }
}
    fn call(&self, inter: &mut Interpreter,arguments:Vec<Value>)->Value {
       match self{
          Func::Native(n_f)=>n_f.call(inter,arguments),
          Func::DeclFn(d_f)=>d_f.call(inter,arguments)
    }
    }
}
pub(crate) trait Fn_init<T>{
        fn new(name:T)->Func;
}
impl Fn_init<&str> for Func{
    fn new(name:&str)->Func {
       Native_Fn::new(name)
    }
}
impl Func {
   pub(crate) fn to_string(&self)->String{
      return  if let Func::Native(_)=self{
            String::from("<native fn>")
        }else{
              String::from("<decl fn>")
        }
    }
}
pub (crate) enum Fn_Type{
    Func,
    Method
}
impl Fn_Type{
 pub (crate) fn to_str(&mut self)->&str{
       return if let _Func=self{
         "Func"
       }else{
        "Method"
       }
   } 
}