use std::cmp::Ordering;
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

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
         if let Value::num(n1)=self{
            if let Value::num(n2)=other{
                return n1==n2;
            }
        }
        panic!("类型错误，不支持此类型比较");
    }
}

impl PartialOrd  for Value{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Value::num(n1)=self{
            if let Value::num(n2)=other{
                return n1.partial_cmp(n2);
            }
        }
        panic!("类型错误，不支持此类型比较");
    }
}