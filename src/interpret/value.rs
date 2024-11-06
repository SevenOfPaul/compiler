use std::ops::{Neg, Not};

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
         _=>panic!("不支持负号操作")
     })
    }
}
impl Not  for Value{
    type Output = Self;

    fn not(self) -> Self::Output {
        Value::bool(match  self {
            Value::bool(n)=>!n,
            _=>panic!("不支持取反操作")
        })
    }
}