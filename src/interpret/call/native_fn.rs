use chrono::Local;

use crate::{error::X_Err, tools::printf};
use super::{Call, Fn_init, Func, Funcs, Interpreter, Value};

#[derive(Debug, Clone)]
pub(crate) struct Native_Fn{
pub(crate) name:String
}
//语言本身的函数
impl Fn_init<&str> for Func{
    fn new(name:&str)->Func {
       Native_Fn::new(name)
    }
}
impl Call for Native_Fn{
    fn arity(&self) -> usize {
         Funcs.get(&self.name).unwrap().0
    }

    fn call(&self, _inter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value,X_Err> {
        //这里得改
       Ok(Funcs.get(&self.name).unwrap().1(arguments))
    }
}
impl Fn_init<&str> for Native_Fn{
    fn new(name:&str)->Func{
          Func::Native( Self{name:String::from(name)})
    }
}