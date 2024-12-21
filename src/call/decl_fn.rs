use crate::{ast::statment::stmt::Stmt, interpret::env::Environment};
use crate::error::X_Err;
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
     match &self.decl.as_ref() {
         Stmt::Func {name:_name,params, body: _body }=>params.len(),
        _=>panic!("类型错误")
     }
    }

    fn call(&self, inter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value,X_Err> {
        let mut env = Environment::new(Some(inter.env.clone()));
        if let Stmt::Func { name:_, params, body } = self.decl.as_ref() {
            for (i, param) in params.iter().enumerate() {
                env.add(param, arguments[i].clone()).unwrap();
            }    
            //调用执行
          let res=inter.execute_block(body, env);
           if let Err(res_val)=res{
               match res_val {
                   X_Err::ret(r)=>Ok(r.val),
                   _=>Err(res_val)
               }
           }else{
               Ok(Value::Nil)
           }
        } else {
            panic!("Type error")
        }
    }
}
impl Fn_init<Stmt> for Decl_Fn{
    fn new(decl:Stmt)->Func{
         Func::Decl(Self{decl:Box::from(decl)})
    }
}
