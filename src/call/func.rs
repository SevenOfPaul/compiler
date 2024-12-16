use super::{Call, Funcs};
#[derive(Debug, Clone)]
pub (crate) struct Func{
name:String
}
impl Func{
   pub(crate) fn new(name:&str)->Self{
           Self{name:String::from(name)}
    }
}
impl Call for Func{
    fn arity(&self) -> usize {
       Funcs.get(&self.name).unwrap().0
    }

    fn call(&self,arguments:Vec<super::Value>)->super::Value {
     Funcs.get(&self.name).unwrap().1(arguments)
    }
}