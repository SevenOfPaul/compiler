use super::{Call, Funcs};
#[derive(Debug, Clone)]
pub (crate) struct Time_Func{
name:String
}
impl Time_Func{
   pub(crate) fn new(name:&str)->Self{
           Self{name:String::from(name)}
    }
}
impl Call for Time_Func{
    fn arity(&self) -> usize {
       Funcs.get(&self.name).unwrap().0
    }

    fn call(&self,arguments:Vec<super::Value>)->super::Value {
     Funcs.get(&self.name).unwrap().1(arguments)
    }
}