mod time;

use crate::interpret::value::Value;

pub(crate) trait Call{
    //检查参数数量
    fn arity(&self) -> usize;
    fn call<T>(&self,arguments:Vec<Value>)->T;
}