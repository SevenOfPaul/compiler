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
    fn expression(&mut self) ->Expr{
        self.equality()
    }
    fn equality(&mut self) ->Expr{
        let mut  expr=   self.comparison();
        while self.fulfill(vec![Token_type::BANG_EQUAL,Token_type::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let r = self.comparison();
            expr = Expr::Binary {
                l_expression: Box::from(expr),
                operator,
                r_expression: Box::from(r)
            }
        }
        expr
    }
    fn comparison(&mut self) ->Expr{
     let mut  expr=self.term();
        while self.fulfill(vec![]) {
            let operator = self.previous();
            let r_expression=self.term();
            expr=Expr::Binary {l_expression:expr,operator,r_expression}
        }
        expr
    }
    fn advance(&mut self)->Token{
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
    fn fulfill(&mut self, types:Vec<Token_type>) ->bool{
        //当前的token 必须是需要的 才能继续
        for t in types{
            if  self.check(t){
                self.advance();
                return true;
            }
        }
        false
    }
    fn is_end(&self)->bool{
        self.previous().token_type==Token_type::EOF
    }
    //当前token
    fn peek(&self)->&Token{
        &self.tokens[self.pos]
    }
    //前一个token
    fn previous(&self)->Token{
        self.tokens[self.pos - 1].clone()
    }

}

