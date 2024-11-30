
/*
添加变量进全局变量池子
*/
use crate::interpret::env::enviroment;

#[macro_export]
macro_rules! env_add(
($key:ident,$val:ident)=>{
{
  enviroment.lock().unwrap().get_mut().add($key,$val)
  }}
);
/*
从全局变量池中获取变量
*/
#[macro_export]
macro_rules! env_get(
($key:ident)=>{
{
 enviroment.lock().unwrap().get_mut().get($key)

  }}
);

/*
修改操作
*/
#[macro_export]
macro_rules! env_set(
($key:ident,$val:ident)=>{
{
  enviroment.lock().unwrap().get_mut().set($key,$val)
}
    }
);

