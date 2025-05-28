pub(crate) mod resolve;
pub(crate) mod fn_type;
use crate::ast::token::token::Token;
use crate::error::X_Err;
use crate::interpret::call::{Fn_init, Func};
use crate::interpret::value::Value;
use crate::interpret::Run_Err;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub(crate) struct Environment {
    pub(crate) enclose: Option<Rc<RefCell<Environment>>>,
    pub(crate) local: HashMap<String, Rc<RefCell<Value>>>,
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
            let rc_val = Rc::new(RefCell::new(val.clone()));
            self.local.insert(key, rc_val);
            Ok(val)
        }
    }
    //赋值 实际上就是修改变量
    pub(crate) fn assign(&mut self, name: &Token, val: Value) -> Result<Value, X_Err> {
        let key = name.clone().lexeme;
        if let Some(rc_val) = self.local.get(&key) {
            *rc_val.borrow_mut() = val.clone();
            return Ok(val);
        }
        if self.enclose.is_some() {
            return self.enclose.as_ref().unwrap().borrow_mut().assign(name, val);
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
        if let Some(rc_val) = self.local.get(&key) {
            Ok(rc_val.borrow().clone())
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
                Rc::new(RefCell::new(Value::Func(Func::new(name.as_str())))),
            );
        });
    }
    ///全局查找
    pub(crate) fn get_by_global(&mut self, name: &Token) -> Result<Value, X_Err> {
        let key = name.clone().lexeme;
        let mut env = self.clone();
        while env.enclose.is_some() {
            env = env.enclose.unwrap().borrow().clone();
        }
        if let Some(rc_val) = env.local.get(&name.lexeme.clone()) {
            Ok(rc_val.borrow().clone())
        } else {
            Err(Run_Err::new(name.clone(), String::from(key + "变量未声明")))
        }
    }
}
