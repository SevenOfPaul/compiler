pub(crate) mod macors;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::interpret::value::Value;
pub(crate) struct Environment{
   pub(crate)   enclose:Option<Box<Environment>>,
    pub(crate)   local:HashMap<String,Value>
}
impl Environment{
    fn new(enclose:Option<Environment>) -> Self{
        if enclose.is_none(){
            Self{enclose:None,local:HashMap::new()}
        }else{
            Self{enclose:Some(Box::from(enclose.unwrap())),local:HashMap::new()}
        }
    }
}
//存储变量的环境
lazy_static! {
    //最高作用域 也是全局作用域
    pub(crate)  static ref enviroment:Mutex<RefCell<Environment>>=Mutex::new(RefCell::new(Environment::new(None)));
}