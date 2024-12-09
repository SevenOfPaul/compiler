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
    /*
    需要先修改
    */
}
pub(crate)  trait Get<T>{
 fn get_value(&self)->Option<T>;
}

impl Get<String> for Object {
   fn get_value(&self) ->Option<String> {
      match self {
          Object::Str(s)=>Some(s.clone()),
        _=>None
      }
    }
}
impl Get<f32> for Object {
    fn get_value(&self) ->Option<f32> {
         match self {
            Object::Num(n)=>Some(*n),
            _=>None
        }
    }

}
impl Get<bool> for Object {
    fn get_value(&self) ->Option<bool> {
         match self {
            Object::Bool(b)=>Some(*b),
            _=>None
        }
    }

}