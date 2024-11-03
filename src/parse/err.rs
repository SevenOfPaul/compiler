use crate::ast::token::token::Token;

pub(crate) struct Parse_Err{
    token:Token,
    mes:String
}
impl Parse_Err{
    pub(crate) fn new(token:Token, mes:String) ->Self{
        Self{token,mes}
    }
}