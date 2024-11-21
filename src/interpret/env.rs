use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::interpret::value::Value;
//存储变量的环境
lazy_static! {
    pub(crate)  static ref Environment:Mutex<RefCell<HashMap<String,Value>>>=Mutex::from(RefCell::from({
       HashMap::new()
    }));
}