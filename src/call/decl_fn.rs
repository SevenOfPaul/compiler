use std::{cell::RefCell, rc::Rc};

use crate::{ast::statment::stmt::Stmt, interpret::env::Environment};

use super::{Call, Value};

pub (crate) struct Func{
pub (crate) decl:Box<Stmt>
}
impl Func{
   pub(crate) fn new(decl:Stmt)->Self{
           Self{decl:Box::from(decl)}
    }
    pub (crate) fn to_string(&self)->String{
         String::from("<native fn>")
    }
}
impl Call for Func{
    fn arity(&self) -> usize {
        todo!()
    }

    fn call(&self,env:Rc<RefCell<Environment>>,arguments:Vec<Value>)->Value {
      let env=Environment::new(Some(env));
      todo!()
    }
}
