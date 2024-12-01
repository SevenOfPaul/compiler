
/*
添加变量进全局变量池子
*/

#[macro_export]
macro_rules! env_add(
($key:ident,$val:ident)=>{
{
 self.env.borrow().add($key,$val)
  }}
);
/*
从全局变量池中获取变量
*/
#[macro_export]
macro_rules! env_get(
($key:ident)=>{
{

    Ok( self.env.borrow().get($key)?)

  }}
);

/*
修改操作
*/
#[macro_export]
macro_rules! env_set(
($key:ident,$val:ident)=>{
{
  self.env.borrow().set($key,$val)
}
    }
);

