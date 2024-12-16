use super::{Call, Funcs};

struct clock{

}
impl Call for clock{
    fn arity(&self) -> usize {
       Funcs.get(&String::from("now")).unwrap().0
    }

    fn call(&self,arguments:Vec<super::Value>)->super::Value {
     Funcs.get(&String::from("now")).unwrap().1(arguments)
    }
}