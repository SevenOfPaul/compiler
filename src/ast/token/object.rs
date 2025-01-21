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
        match (self, other) {
            (Object::Num(n1), Object::Num(n2)) => n1.eq(n2),
            (Object::Bool(b1), Object::Bool(b2)) => b1.eq(b2),
            (Object::Nil, Object::Nil) => true,  // Nil 应该等于 Nil
            (Object::Str(s1), Object::Str(s2)) => s1.eq(s2),
            _ => false
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
            Object::Str(s) => s.hash(state),
            Object::Num(n) =>{
                n.to_bits().hash(state)
            },
            Object::Bool(b) => b.hash(state),
            Object::Nil => 0.hash(state),
        }
    }
}