#[derive(Debug,Clone)]
pub(crate) enum Object {
    str(String),
    num(f32),
    boolean(bool),
}
impl Object{
   pub(crate) fn to_string(&self)->String{
       match self {
           Object::num(n)=>{
               n.to_string()
           },Object::boolean(b)=>{
               b.to_string()
           }
           _=>{
               self.to_string()
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
     return match self {
          Object::str(s)=>Some(s.clone()),
        _=>None
      }
    }
}
impl Get<f32> for Object {
    fn get_value(&self) ->Option<f32> {
        return match self {
            Object::num(n)=>Some(*n),
            _=>None
        }
    }

}
impl Get<bool> for Object {
    fn get_value(&self) ->Option<bool> {
        return match self {
            Object::boolean(b)=>Some(*b),
            _=>None
        }
    }

}