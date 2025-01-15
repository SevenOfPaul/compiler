pub(crate) mod resolve;
use crate::ast::token::token::Token;
use crate::error::X_Err;
use crate::interpret::call::{Fn_init, Func};
use crate::interpret::value::Value;
use crate::interpret::Run_Err;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
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
    pub (crate) fn assign_at(&mut self,distance: i32, name: &Token, val: Value)->Result<Value,X_Err>{
                 self.ancestor(distance).add(name,val)
    }
    pub (crate) fn ancestor(&mut self,distance: i32)->Box<Environment>{
                let mut env = self.clone();
                for _ in 0..distance {
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
    pub(crate) fn set(&mut self, name: &Token, val: Value) -> Result<Value, X_Err> {
        let key = name.clone().lexeme;
        let res = self.local.contains_key(&key);
        if res {
            self.local.insert(key, val.clone());
            Ok(val)
        } else if self.enclose.is_some() {
            self.enclose.clone().unwrap().borrow_mut().set(name, val)
        } else {
            Err(Run_Err::new(name.clone(), String::from(key + "变量未声明")))
        }
    }
}
//存储变量的环境
