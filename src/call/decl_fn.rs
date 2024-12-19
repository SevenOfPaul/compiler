use crate::{ast::statment::stmt::Stmt, interpret::env::Environment};

use super::{Call, Fn_init, Func, Interpreter, Value};
/**自定义的函数 */
#[derive(Debug, Clone)]
pub struct Decl_Fn{
pub  decl:Box<Stmt>
}
//自定义函数
impl Fn_init<Stmt> for Func{
    fn new(decl:Stmt)->Func {
       Decl_Fn::new(decl)
    }
}
impl Call for Decl_Fn{
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
impl Fn_init<Stmt> for Decl_Fn{
    fn new(decl:Stmt)->Func{
         Func::DeclFn(Self{decl:Box::from(decl)})
    }
}
