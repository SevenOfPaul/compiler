use std::ops::Neg;

pub (crate) enum Value{
    str(String),
    num(f32),
    bool(bool),
    nil
}
impl Neg  for Value{
    type Output = Self;

    fn neg(self) -> Self::Output {
     Value::num(match  self {
         Value::num(n)=>-n,
         _=>panic!("不支持符号操作")
     })
    }
}