use crate::{ast::statment::stmt::Stmt, interpret::env::Environment};

use super::{Call, Interpreter, Value};

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

    fn call(&self, inter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let mut env = Environment::new(Some(inter.env.clone()));
        if let Stmt::Func { name, params, body } = self.decl.as_ref() {
            for (i, param) in params.iter().enumerate() {
                env.add(param, arguments[i].clone()).unwrap();
            }    
            //调用执行
           inter.execute_block(body, env);
           Value::Nil
        } else {
            panic!("Type error")
        }
    }
}
