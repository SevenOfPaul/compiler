use std::collections::HashMap;
use crate::ast::expression::expr::Expr;
use crate::ast::statment::stmt::Stmt;
use crate::interpret::value::Value;
use super::Token;

#[derive(Debug,Clone)]
pub (crate) struct Prototype {
    pub struct_name:Box<Expr>,
    pub fns: HashMap<Token,Stmt>,
    pub body: HashMap<Token,Stmt>,
}
impl Prototype{
    pub(crate) fn new(struct_name:Box<Expr>)->Prototype{
    Self{
        struct_name,
        fns:HashMap::new(),
        body:HashMap::new()
    }
    }
     pub(crate) fn to_string(&self) ->String{
         format!("struct {:?}",self.struct_name)
    }
}
pub trait Property{
    fn get(&self,name:&Token)->Value;
    fn set(&self,name:&Token,val:Value);
}
impl Property for Prototype{
    fn get(&self, name: &Token) -> Value {
        todo!()
    }
    fn set(&self, name: &Token, val: Value) {
        todo!()
    }
}