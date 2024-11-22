use crate::interpret::env::Environment;
use crate::interpret::error::Run_Err;

#[macro_export]
macro_rules! env_add(
($key:ident,$val:ident)=>{
{

    let key=$key.lexeme.clone();
    if Environment.lock().unwrap().get_mut().contains_key(&key){
        Err(Run_Err::new($key.clone(),String::from("变量已存在，不可重复声明")))
    }else{
        Environment.lock().unwrap().get_mut().insert(key.clone(),$val);
        Ok(())
    }
  }}
);
#[macro_export]
macro_rules! env_get(
($key:ident)=>{
{

    let key=$key.lexeme.clone();
    if let Some(v)=Environment.lock().unwrap().borrow().get(&key){
        return Ok(v.clone());
    }else{
     Err(Run_Err::new($key.clone(),String::from(key+"未定义")))
    }
  }}
);

