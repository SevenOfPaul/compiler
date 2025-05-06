use std::collections::HashMap;
use crate::ast::expression::expr::Expr;
use crate::ast::statment::stmt::Stmt;
use crate::interpret::value::Value;
use super::Token;

#[derive(Debug,Clone)]
pub (crate) struct Prototype {
    pub struct_name:Box<Expr>,
    pub fns: HashMap<Token,Stmt>,
    pub fields: HashMap<Token,Value>,
}
impl Prototype{
    pub(crate) fn new(struct_name:Box<Expr>)->Prototype{
    Self{
        struct_name,
        fns:HashMap::new(),
        fields:HashMap::new()
    }
    }
     pub(crate) fn to_string(&self) ->String{
        let mut  str=String::new();
        if let Expr::Struct { name, fields  }=self.struct_name.as_ref(){
          let key_val_pairs: Vec<String> =self.fields.iter()
                .map(|(key, val)| format!("{:?}:{:?}", key, val))
                .collect();
            // 将所有 key:val 对用逗号连接
            let key_val_str = key_val_pairs.join(",");
            // 构造最终的格式化字符串
            str = format!("Instance {} {{ {} }}", name.lexeme, key_val_str);
        }
        str
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