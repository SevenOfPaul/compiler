pub(crate) mod resolve;
use crate::ast::token::token::Token;
use crate::error::X_Err;
use crate::interpret::call::{Fn_init, Func};
use crate::interpret::value::Value;
use crate::interpret::Run_Err;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::tools::println;

#[derive(Clone, Debug)]
pub(crate) struct Environment {
    pub(crate) enclose: Option<Rc<RefCell<Environment>>>,
    pub(crate) local: HashMap<String, Value>,
}
impl Environment {
    pub(crate) fn new(enclose: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclose,
            local: HashMap::new(),
        }
    }
    //添加变量
    pub(crate) fn add(&mut self, name: &Token, val: Value) -> Result<Value, X_Err> {
        let key = name.clone().lexeme;
        if self.local.contains_key(&key) {
            Err(Run_Err::new(
                name.clone(),
                String::from("变量已存在，不可重复声明"),
            ))
        } else {
            self.local.insert(key, val.clone());
            Ok(val)
        }
    }
    //赋值 实际上就是修改变量
        pub(crate) fn assign(&mut self, name: &Token, val: Value) -> Result<Value, X_Err> {
        let key = name.clone().lexeme;
        if self.local.contains_key(&key) {
              *self.local.get_mut(&key).unwrap() = val.clone();
               return Ok(val.clone())
        }  

         if self.enclose.is_some() {
           return self.enclose.as_ref().unwrap().borrow_mut().assign(name, val)
         }
           Err(Run_Err::new(
                name.clone(),
                String::from("变量未定义")
            ))
           
        
    }
    pub (crate) fn assign_at(&mut self,distance: i32, name: &Token, val: Value)->Result<Value,X_Err>{
        self.ancestor(distance).assign(name,val)
    }
    pub (crate) fn ancestor(&mut self,distance: i32)->Box<Environment>{
                let mut env = self.clone();
                for _ in 1..distance {
                    env = env.enclose.unwrap().borrow().clone();
                }
                Box::new(env)
    }
    pub(crate) fn get_at(&mut self, distance: i32, key: &Token) -> Result<Value, X_Err> {
        self.ancestor(distance).get(key)
    }
    pub(crate) fn get(&self, name: &Token) -> Result<Value, X_Err> {
        let key = name.clone().lexeme;
        let res = self.local.get(&key);
        if res.is_some() {
            Ok(res.unwrap().clone())
        } else if self.enclose.is_some() {
            self.enclose.as_ref().unwrap().borrow().get(name)
        } else {
            Err(Run_Err::new(name.clone(), String::from(key + "未定义")))
        }
    }
    pub(crate) fn init_fn(&mut self, names: Vec<&String>) {
        names.into_iter().for_each(|name| {
            self.local.insert(
                String::from(name),
                //这里需要添加一个标识识别native or decl
                Value::Func(Func::new(name.as_str())),
            );
        });
    }
    pub(crate) fn get_by_global(&mut self, name: &Token) -> Result<Value, X_Err> {
        let key = name.clone().lexeme;
        let mut env =self.clone();
        while env.enclose.is_some() {
          env=env.enclose.unwrap().borrow().clone();
        }
        if let Some(v)=env.local.get(&name.lexeme.clone()){
                Ok(v.clone())
        }else{
            Err(Run_Err::new(name.clone(), String::from(key + "变量未声明")))
        }

    }
}
//存储变量的环境
