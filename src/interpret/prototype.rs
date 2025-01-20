use crate::ast::statment::stmt::Stmt;

use super::Token;

#[derive(Debug,Clone)]
pub (crate) struct Prototype {
    pub name: Token,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}
impl Prototype{
    pub(crate) fn new(name:Token)->Prototype{
    Self{
        name,
        params:Vec::new(),
        body:Vec::new()
    }
    }
     pub(crate) fn to_string(&self) ->String{
        return format!("struct {:?}",self.name.lexeme);
    }
}