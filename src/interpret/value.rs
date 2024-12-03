use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone)]
pub(crate) enum Value {
    str(String),
    num(f32),
    bool(bool),
    nil,
}
impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Value::num(match self {
            Value::num(n) => -n,
            _ => panic!("不支持负号操作"),
        })
    }
}
impl Value {
    pub(crate) fn is_num(&self) -> bool {
        match self {
            Value::num(n) => true,
            _ => false,
        }
    }
    pub(crate) fn is_str(&self) -> bool {
        match self {
            Value::str(s) => true,
            _ => false,
        }
    }
}
impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if let Value::num(n1) = self {
            if let Value::num(n2) = other {
                return Value::num(n1 + n2);
            } else {
                panic!("不支持此类型加法操作");
            }
        } else if let Value::str(s1) = self {
            if let Value::str(s2) = other {
                return Value::str(s1 + &s2);
            } else if let Value::num(n2) = other {
                return Value::str(s1 + &n2.to_string());
            } else {
                panic!("不支持此类型加法操作");
            }
        }
        panic!("不支持此类型加法操作");
    }
}
impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if let Value::num(n1) = self {
            if let Value::num(n2) = other {
                return Value::num(n1 - n2);
            }
        }
        panic!("不支持此类型减法操作");
    }
}
impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if let Value::num(n1) = self {
            if let Value::num(n2) = other {
                return Value::num(n1 * n2);
            }
        }
        panic!("不支持此类型乘法操作");
    }
}
impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if let Value::num(n1) = self {
            if let Value::num(n2) = other {
                return Value::num(n1 / n2);
            }
        }
        panic!("不支持此类型法操作");
    }
}
impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        Value::bool(match self {
            Value::bool(n) => !n,
            _ => panic!("不支持取反操作"),
        })
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        println!("{:?}",self);
       return  match self {
            Value::num(n1) => {
                if let Value::num(n2) = other {
                    n1 == n2
                } else {
                    false
                }
            }
            Value::bool(b1)=>{
                if let Value::bool(b2)=other{
                      *b1==*b2
                }else{
                    *b1
                }
            }
            Value::nil => false,
            _ => {
                panic!("类型错误，不支持此类型比较");
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Value::num(n1) = self {
            if let Value::num(n2) = other {
                return n1.partial_cmp(n2);
            }
        }
        panic!("类型错误，不支持此类型比较");
    }
}
