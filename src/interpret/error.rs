use crate::ast::token::token::Token;

#[derive(Debug)]
pub(crate) struct Run_Err{
    token:Token,
    mes:String
}
impl Run_Err{
    pub(crate) fn new(token:Token, mes:String) ->Self{
        Self{token,mes}
    }
}