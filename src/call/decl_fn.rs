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
      let mut env=Environment::new(Some(env));
      if let Stmt::Func { name, params, body }=self.decl.as_ref(){
      for argu in params.iter().enumerate(){
         env.add(argu.1, arguments[argu.0].clone());
      }
      //执行他
      
    }else{
        panic!("类型错误")
    }
      todo!()
    }
}
