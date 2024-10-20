#[derive(Debug,Clone)]
pub(crate) enum Object {
    str(String),
    number(f32)
}
impl Object{
   pub(crate) fn to_string(&self)->String{
        if let Object::number(num) = self{
            num.to_string()
        }else{
            self.to_string()
        }
    }
}