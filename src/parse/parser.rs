use crate::ast::expr::Expr;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_type;

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
    fn advance(&mut self)->&Token{
        if !self.is_end(){
            self.pos += 1;
        }
         self.previous()
    }
    fn check(&self,token_type:Token_type)->bool{
        if self.is_end(){
              false
        }else {
            (*self.peek()).token_type == token_type
        }
    }
    fn is_end(&self)->bool{
        (*self.previous()).token_type==Token_type::EOF
    }
    //当前token
    fn peek(&self)->&Token{
        &self.tokens[self.pos]
    }
    //前一个token
    fn previous(&self)->&Token{
        &self.tokens[self.pos - 1]
    }

}
