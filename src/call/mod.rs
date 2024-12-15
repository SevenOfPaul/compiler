use crate::ast::token::object::Object;

pub(crate) trait Call{
    //检查参数数量
    fn arity(&self) -> usize;
    fn call(&self,arguments:Vec<Object>);
}