use crate::{
    interpret::{interpreter::Interpreter, value::Value},
    tools::printf,
};
mod decl_fn;
mod native_fn;
use crate::error::X_Err;
use chrono::prelude::*;
use decl_fn::Decl_Fn;
use lazy_static::lazy_static;
use native_fn::Native_Fn;
/**语言内部自带的函数 */
use std::collections::HashMap;
type Native_Fn_Trait=dyn Fn(Vec<Value>)->Value+Send+Sync+'static;
lazy_static! {
        ///内置函数列表 使用hashmap存储 存储一个元组 0为函数名 1为参数数量 2为函数实现
    pub(crate) static ref Funcs: HashMap<String, (usize,Box<Native_Fn_Trait>)> = {
        HashMap::from([(
            String::from("String"),
            (1, Box::new(|arguments: Vec<Value>| {
                Value::Str(arguments[0].to_string())
            }) as Box<Native_Fn_Trait>)
        ),(
            String::from("TypeOf"),
            (1, Box::new(|arguments: Vec<Value>| {
                arguments[0].get_type()
            }) as Box<Native_Fn_Trait>)),
            (String::from("now"),
               (0, Box::new(|_arguments| {
                 let now = Local::now();
                     Value::Time(now)
                }) as Box<Native_Fn_Trait>)
            ),    (String::from("PP"),
               (0, Box::new(|_arguments| {
                    printf("==================================");
                    printf("==  PP 在此   =====================");
                    printf("==================================");
                    Value::Nil
                }) as Box<Native_Fn_Trait>)
            )
        ])
    };
}
#[derive(Debug, Clone)]
pub(crate) enum Func {
    Native(Native_Fn),
    Decl(Decl_Fn),
}
pub(crate) trait Call {
    //检查参数数量
    fn arity(&self) -> usize;
    fn call(&self, inter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, X_Err>;
} // Box<dyn Fn(Vec<Value>) -> Value + Send + Sync + 'static>
impl Call for Func {
    fn arity(&self) -> usize {
        match self {
            Func::Native(n_f) => n_f.arity(),
            Func::Decl(d_f) => d_f.arity(),
        }
    }
    fn call(&self, inter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, X_Err> {
        match self {
            Func::Native(n_f) => n_f.call(inter, arguments),
            Func::Decl(d_f) => d_f.call(inter, arguments),
        }
    }
}
pub(crate) trait Fn_init<T> {
    fn new(func: T) -> Func;
}
impl Func {
    pub(crate) fn to_string(&self) -> String {
        if let Func::Native(_) = self {
            String::from("<native fn>")
        } else {
            String::from("<decl fn>")
        }
    }
}
pub(crate) enum Fn_Type {
    Func,
    Method,
}
impl Fn_Type {
    pub(crate) fn to_str(&mut self) -> &str {
        return if let Fn_Type::Func = self {
            "Func"
        } else {
            "Method"
        };
    }
}
