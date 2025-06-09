use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use chrono::{DateTime, Local};
use crate::ast::expression::expr::Expr;
use crate::ast::token::token::Token;
use crate::interpret::call::Call;
use crate::interpret::call::Func;
use crate::interpret::prototype::Prototype;
use crate::error::X_Err;
use self::interpreter::Interpreter;
use super::interpreter;

#[derive(Debug, Clone)]
pub(crate) enum Value {
    Str(String),
    Num(f32),
    Bool(bool),
    Time(DateTime<Local>),
    Func(Func),
    Struct{name:Token},
    Instance(Prototype),
    Nil,
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Num(n1), Value::Num(n2)) => Value::Num(n1 + n2),
            (Value::Str(s1), Value::Str(s2)) => Value::Str(s1 + &s2),
            (Value::Str(s1), Value::Num(n2)) => Value::Str(s1 + &n2.to_string()),
            (Value::Time(t1), Value::Num(n2)) => Value::Time(t1.add(chrono::Duration::seconds(n2 as i64))),
            _ => panic!("不支持此类型加法操作")
        }
    }
}
impl Call for Value{
    fn arity(&self) -> usize {
       if let Value::Func(f)=self{
        f.arity()
      }else{
        panic!("只支持函数调用") 
      }
    }

    fn call(&self,inter: &mut Interpreter,arguments:Vec<Value>)->Result<Value,X_Err> {
        if let Value::Func(f)=self{
             f.call(inter, arguments)
       }else{
        panic!("只支持函数调用")
       }
    }
    
}
impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Num(n1), Value::Num(n2)) => Value::Num(n1 / n2),
            _ => panic!("不支持此类型除法操作")
        }
    }
}
impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Num(n1), Value::Num(n2)) => Value::Num(n1 * n2),
            _ => panic!("不支持此类型乘法操作")
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Value::Bool(!b),
            _ => panic!("不支持取反操作"),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Num(n) => Value::Num(-n),
            _ => panic!("不支持负号操作"),
        }
    }
}
impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Num(n1), Value::Num(n2)) => n1 == n2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Time(t1), Value::Time(t2)) => t1.eq(t2),
            (Value::Nil, Value::Nil) => true,
            _ => panic!("类型错误，不支持此类型比较")
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Num(n1), Value::Num(n2)) => n1.partial_cmp(n2),
            (Value::Time(t1), Value::Time(t2)) => t1.partial_cmp(t2),
            _ =>panic!("类型错误，不支持此类型比较")
        }
    }
}
impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Num(n1), Value::Num(n2)) => Value::Num(n1 - n2),
            (Value::Time(t1), Value::Time(t2)) => 
            Value::Num(t1.signed_duration_since(t2).num_milliseconds() as f32),
            _ => panic!("不支持此类型减法操作")
        }
    }
}
impl Value {
    pub (crate) fn get_type(&self) -> Value {
        match self {
            Value::Num(_) => Value::Str(String::from("Num")),
            Value::Bool(_) => Value::Str(String::from("Bool")),
            Value::Str(_) => Value::Str(String::from("Str")),
            Value::Time(_) => Value::Str(String::from("Time")),
            Value::Func(_) => Value::Str(String::from("Function")),
            Value::Struct{name} => Value::Str(format!("Struct {}", name.lexeme)),
            Value::Instance(p) => {
                if let Expr::Struct { name, fields: _ } = p.struct_name.as_ref() {
                    return Value::Str(format!("Instance of {}", name.lexeme));
                }else{
                    return Value::Str(String::from("Instance"));
                }
            }
            Value::Nil => Value::Str(String::from("Nil")),
        }
        
    }
    pub(crate) fn is_num(&self) -> bool {
        match self {
            Value::Num(n) => true,
            _ => false,
        }
    }
    pub(crate) fn is_str(&self) -> bool {
        match self {
            Value::Str(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_time(&self) -> bool {
        match self {
            Value::Time(t) => true,
            _ => false,
        }
    }
    pub (crate) fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Num(n)=>*n==1.0,
             _=>false
        }
    }
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Num(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Time(t) => write!(f, "{}", t),
            Value::Func(func) => write!(f, "{}", func.to_string()),
            Value::Struct{name} => write!(f,"struct {}",name.lexeme),
            Value::Instance(p) => write!(f, "{}", p.to_string()),
            Value::Nil => write!(f, "nil"),
        }
    }
}





