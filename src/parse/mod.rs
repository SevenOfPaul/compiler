use crate::ast::token::token::Token;
use crate::error;
use crate::error::{ X_Err};
pub(crate) mod parser;
#[derive(Debug)]
pub(crate) struct Parse_Err{
    pub(crate) token:Token,
    pub(crate) mes:String
}
impl Parse_Err{
    fn new(token:Token, mes:String) ->X_Err{
        error::log(token.line, &token.lexeme, &mes);
        X_Err::parse(Parse_Err{token,mes})
    }
}
#[derive(Debug)]
pub (crate) struct Break{

}
impl Break{
 pub (crate) fn new() ->X_Err{
       X_Err::brk(Break{})
    }
}
#[derive(Debug)]
pub (crate) struct Continue{

}
impl Continue{
 pub (crate) fn new() ->X_Err{
       X_Err::cte(Continue{})
    }
}