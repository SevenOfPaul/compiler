use crate::interpret::value::Value;

pub(crate) fn printf(v:Value){
    match v {
        Value::Num(num) => println!("{}", num),
        Value::Str(s)=>println!("{}", s),
        Value::Bool(b)=>println!("{}", b),
        Value::Time(t)=>println!("{:?}", t),
         Value::Func(f)=>println!("{:?}", f.to_string()),
        _=>println!("{:?}",Value::Nil)
    }
}