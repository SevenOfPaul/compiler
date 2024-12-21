mod report;

pub use report::*;
use crate::ast::token::token::Token;
use crate::error;
use crate::interpret::value::Value;

#[derive(Debug)]
pub (crate) enum X_Err{
    parse(Parse_Err),
    run(Run_Err),
    ret(Return)
}
#[derive(Debug)]
pub(crate) struct Parse_Err{
    token:Token,
    pub(crate) mes:String
}
impl Parse_Err{
    pub(crate) fn new(token:Token, mes:String) ->Self{
        error::log(token.line, &token.lexeme, &mes);
        Self{token,mes}
    }
}
#[derive(Debug)]
pub(crate) struct Run_Err{
    pub(crate) token:Token,
    pub(crate) mes:String
}
trait Log<T>{

}
impl  X_Err{}
impl Run_Err{
    pub(crate) fn new(token:Token, mes:String) ->Self{
        error::log(token.line, &token.lexeme, &mes);
        Self{token,mes}
    }
}
#[derive(Debug)]
pub(crate) struct Return{
    pub(crate) val:Value,
}