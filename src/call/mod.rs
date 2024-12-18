use std::{cell::RefCell, rc::Rc};

use crate::interpret::{env::Environment, interpreter::Interpreter, value::Value};

pub (crate) mod native_fn;
pub (crate) mod decl_fn;
pub(crate) trait Call{
    //检查参数数量
    fn arity(&self) -> usize;
    fn call(&self,env:Rc<RefCell<Environment>>,arguments:Vec<Value>)->Value;
}// Box<dyn Fn(Vec<Value>) -> Value + Send + Sync + 'static>
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