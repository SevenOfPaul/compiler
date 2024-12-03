use crate::error;
use crate::ast::token::token::Token;

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