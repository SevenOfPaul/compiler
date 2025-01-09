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