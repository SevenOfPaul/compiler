use crate::ast::expr::Expr;
use crate::ast::token::token::Token;

pub(crate) struct Parser{
    tokens: Vec<Token>,
    pos:usize
}
impl Parser{
    fn new(tokens:Vec<Token>)->Self{
        Self{tokens,pos:0}
    }
    fn expression(&self)->Expr{
        self.equality()
    }
    fn equality(&self)->Expr{

    }
}