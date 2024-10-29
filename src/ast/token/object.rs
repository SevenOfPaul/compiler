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
    pub(crate) fn get_value(&self)->String{
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
}