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
    /**/
    fn expression(&mut self) ->Expr{
        self.equality()
    }
    fn equality(&mut self) ->Expr{
        let mut  expr=   self.comparison();
        while self.fulfill(vec![Token_type::BANG_EQUAL,Token_type::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let r_expression =Box::from( self.comparison());
            expr = Expr::Binary {
                l_expression: Box::from(expr),
                operator,
                r_expression
            }
        }
        expr
    }
    fn comparison(&mut self) ->Expr{
     let mut  expr=self.term();
        while self.fulfill(vec![Token_type::LESS,Token_type::GREATER,Token_type::LESS_EQUAL,Token_type::GREATER_EQUAL]) {
            let operator = self.previous();
            let r_expression=Box::from(self.term());
            expr=Expr::Binary {l_expression:Box::from(expr),operator,r_expression}
        }
        expr
    }
    fn term(&mut self)->Expr{
        let mut expr=self.factor();
        while self.fulfill(vec![Token_type::MINUS,Token_type::PLUS]) {
           let operator = self.previous();
            let r_expression=Box::from(self.factor());
            expr=Expr::Binary {l_expression: Box::from(expr),operator,r_expression}
        }
        expr
    }
    fn factor(&mut self)->Expr{
    let mut expr=Expr::Unary { operator: Token::new(Token_type::NIL,String::new(),None,0),
        r_expression:Box::from(Expr::Literal{val:None}) };
        while self.fulfill(vec![Token_type::SLASH,Token_type::STAR]){
            let operator = self.previous();
            let r_expression=Box::from(Expr::Unary { operator: Token::new(Token_type::NIL,String::new(),None,0), 
                r_expression:Box::from(Expr::Literal{val:None}) });
            expr=Expr::Binary {l_expression: Box::from(expr),operator,r_expression};

        }
    expr
    }
    /*止*/
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

