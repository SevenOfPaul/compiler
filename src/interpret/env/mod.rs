pub(crate) mod macors;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::ast::expression::expr::Expr;
use crate::ast::token::token::Token;
use crate::interpret::error::Run_Err;
use crate::interpret::value::Value;
#[derive(Clone)]
pub(crate) struct Environment{
   pub(crate)   enclose:Option<Box<Environment>>,
    pub(crate)   local:HashMap<String,Value>
}
impl Environment{
    pub(crate) fn new(enclose: Option<Environment>) -> Self{
        if enclose.is_none(){
            Self{enclose:None,local:HashMap::new()}
        }else{
            Self{enclose:Some(Box::from(enclose.unwrap())),local:HashMap::new()}
        }
    }
    pub(crate)  fn get(&self,name:&Token)->Result<Value,Run_Err>{
        let key=name.clone().lexeme;
        let res= self.local.get(&key);
        if res.is_some(){
            Ok(res.unwrap().clone())
        }else if self.enclose.is_some() {
            self.enclose.as_ref().unwrap().get(name)
        }else{
            Err(Run_Err::new(name.clone(),String::from(key+"未定义")))
        }
    }
    pub(crate) fn set(&mut self, name:&Token, val:Value) ->Result<Value,Run_Err>{
    let key=name.clone().lexeme;
    let res= self.local.contains_key(&key);
    if res {
        self.local.insert(key, val.clone());
        Ok(val)
    }else if self.enclose.is_some() {
        self.enclose.clone().unwrap().set(name, val)
    }else{
        Err(Run_Err::new(name.clone(),String::from(key+"变量未声明")))
    }
    }
    pub(crate) fn add(&mut self, name:&Token, val:Value) ->Result<(),Run_Err>{
        let key=name.clone().lexeme;
         if self.local.contains_key(&key){
             Err(Run_Err::new(name.clone(),String::from("变量已存在，不可重复声明")))
         }else{
             self.local.insert(key,val);
             Ok(())
         }
    }
}
//存储变量的环境