use std::hash::{Hash, Hasher};
#[derive(Debug,Clone)]
pub(crate) enum Object {
    Str(String),
    Num(f32),
    Bool(bool),
    Nil
}
impl Object{
   pub(crate) fn to_string(&self)->String{
       match self {
           Object::Num(n)=>{
               n.to_string()
           },Object::Bool(b)=>{
               b.to_string()
           }
           Object::Nil =>{
              String::from("Nil")
           }
           Object::Str(s)=>{
               s.clone()
           }
       }
    }
}


impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
       match self {
              Object::Num(n1)=>{
                if let Object::Num(n2)=other{
                     n1.eq(n2)
                }else{
                     false
                }
              },Object::Bool(b1)=>{
                if let Object::Bool(b2)=other{
                     b1.eq(b2)
                }else{
                     false
                }
              }
              Object::Nil =>{
               false
              }
              Object::Str(s1)=>{
                if let Object::Str(s2)=other{
                     s1.eq(s2)
                }else{
                     false
                }
              }
       }
    }
    
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Str(_) => todo!(),
            Object::Num(_) => todo!(),
            Object::Bool(_) => todo!(),
            Object::Nil => todo!(),
        }
    }
}