use std::collections::HashMap;
use crate::ast::expression::expr::Expr;
use crate::ast::statment::stmt::Stmt;
use crate::interpret::value::Value;
use super::Token;

#[derive(Debug,Clone)]
pub (crate) struct Prototype {
    pub struct_name:Box<Expr>,
    pub fields: HashMap<String,Value>,
}

impl Prototype{
    pub(crate) fn new(struct_name:Box<Expr>,keys:&Vec<Token>,vals:&Vec<Value>)->Prototype{
    Self{
        struct_name,
        fields: keys.iter().zip(vals.iter()).map(|(k,v)| (k.lexeme.clone(), v.clone())).collect(),
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
    fn set(&mut self,name:&Token,val:Value);
}
impl Property for Prototype{
    fn get(&self, name: &Token) -> Value {
        println!("get {:?}",self.fields);
        if let Some(val) = self.fields.get(&name.lexeme) {
            val.clone() // 确保返回值是一个新的 Value 实例 
        }else{
            Value::Nil
        }
    }
    fn set(&mut self, name: &Token, val: Value) {
        println!("set1 prototype {:?}=={:?}=={:?}",self.fields,name.lexeme,val);
        self.fields.insert(name.lexeme.clone(), val); 
        println!("set2 prototype {:?}",self.fields);
    }
}