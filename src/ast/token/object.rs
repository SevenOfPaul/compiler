#[derive(Debug,Clone)]
pub(crate) enum Object {
    str(String),
    num(f32)
}
impl Object{
   pub(crate) fn to_string(&self)->String{
        if let Object::num(n) = self{
            n.to_string()
        }else{
            self.to_string()
        }
    }
}