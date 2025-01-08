use crate::ast::token::token::Token;
use crate::error;
use crate::error::X_Err;
use crate::interpret::value::Value;

pub(crate) mod env;
pub(crate) mod interpreter;
pub(crate) mod value;
pub mod call;

#[derive(Debug)]
pub(crate) struct Run_Err{
    pub(crate) token:Token,
    pub(crate) mes:String
}
impl Run_Err{
 fn new(token:Token, mes:String) ->X_Err{
        error::log(token.line, &token.lexeme, &mes);
        X_Err::run(Run_Err{token,mes})
    }
}
#[derive(Debug)]
pub(crate) struct Return{
    pub(crate) val:Value,
}
impl Return{
    fn new(val:Value) ->X_Err{
        X_Err::ret(Return{val})
    }
}