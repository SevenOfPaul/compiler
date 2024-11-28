
/*
添加变量进全局变量池子
*/
use crate::interpret::env::enviroment;

#[macro_export]
macro_rules! env_add(
($key:ident,$val:ident)=>{
{

    let key=$key.lexeme.clone();
    if enviroment.lock().unwrap().get_mut().local.contains_key(&key){
        Err(Run_Err::new($key.clone(),String::from("变量已存在，不可重复声明")))
    }else{
        enviroment.lock().unwrap().get_mut().local.insert(key.clone(),$val.clone());
        Ok(())
    }
  }}
);
/*
从全局变量池中获取变量
*/
#[macro_export]
macro_rules! env_get(
($key:ident)=>{
{

    let key=$key.lexeme.clone();
    if let Some(v)=enviroment.lock().unwrap().borrow().local.get(&key){
        return Ok(v.clone());
    }else{
        
     Err(Run_Err::new($key.clone(),String::from(key+"未定义")))
    }
  }}
);

/*
修改操作
*/
#[macro_export]
macro_rules! env_set(
($key:ident,$val:ident)=>{
{

    let key=$key.lexeme.clone();
    if enviroment.lock().unwrap().get_mut().local.contains_key(&key){

         enviroment.lock().unwrap().get_mut().local.insert(key,$val.clone());
        Ok($val)
    }else{
         Err(Run_Err::new($key.clone(),String::from(key+"变量未声明")))

    }
  }}
);

