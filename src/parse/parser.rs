use crate::error;
use crate::ast::expr::Expr;
use crate::ast::token::object::{Get, Object};
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_type;
use crate::parse::err::Parse_Err;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    fn parse(&mut self) ->Expr{
      if let Ok(expr)=self.expression(){
          expr
      }else{
         Expr::Literal {val:Some(Object::nil)}
      }
    }
    /*
    执行单操作符
    */
    fn expression(&mut self) -> Result<Expr,Parse_Err>  {
        self.equality()
    }
    //看看等号的运算符
    fn equality(&mut self) -> Result<Expr,Parse_Err>  {
        let mut expr = self.comparison();
        while self.match_token(&[Token_type::BANG_EQUAL, Token_type::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let r_expression = Box::from(self.comparison()?);
            expr = Ok(Expr::Binary {
                l_expression: Box::from(expr?),
                operator,
                r_expression,
            })
        }
        expr
    }
    //或大或小
    fn comparison(&mut self) -> Result<Expr,Parse_Err>  {
        let mut expr = self.term();
        while self.match_token(&[
            Token_type::LESS,
            Token_type::GREATER,
            Token_type::LESS_EQUAL,
            Token_type::GREATER_EQUAL,
        ]) {
            let operator = self.previous();
            let r_expression = Box::from(self.term()?);
            expr = Ok(Expr::Binary {
                l_expression: Box::from(expr?),
                operator,
                r_expression,
            })
        }
        expr
    }
    //是不是加减
    fn term(&mut self) -> Result<Expr,Parse_Err>  {
        let mut expr = self.factor();
        while self.match_token(&[Token_type::MINUS, Token_type::PLUS]) {
            let operator = self.previous();
            let r_expression = Box::from(self.factor()?);
            expr = Ok(Expr::Binary {
                l_expression: Box::from(expr?),
                operator,
                r_expression,
            })
        }
        expr
    }
    //是不是乘除
    fn factor(&mut self) -> Result<Expr,Parse_Err>  {
        let mut expr = self.unary();
        while self.match_token(&[Token_type::SLASH, Token_type::STAR]) {
            let operator = self.previous();
            let r_expression = Box::from(self.unary()?);
            expr = Ok(Expr::Binary {
                l_expression: Box::from(expr?),
                operator,
                r_expression,
            });
        }
        expr
    }
    //
    fn unary(&mut self) -> Result<Expr,Parse_Err>  {
        if self.match_token(&[Token_type::BANG, Token_type::MINUS]) {
            let operator = self.previous();
            let r_expression = Box::from(self.unary()?);
           return Ok(Expr::Unary {
                operator,
                r_expression,
            });
        }
        self.primary()
    }
    //非运算符的情况下
    //进行递归
    fn primary(&mut self) -> Result<Expr,Parse_Err> {
        if self.match_token(&[Token_type::NIL]) {
            Ok(Expr::Literal { val: Some(Object::nil) })
        } else if self.match_token(&[Token_type::TRUE]) {
             Ok(Expr::Literal {
                val: Some(Object::boolean(true)),
            })
        } else if self.match_token(&[Token_type::FALSE]) {
             Ok(Expr::Literal {
                val: Some(Object::boolean(false)),
            })
        } else if self.match_token(&[Token_type::NUMBER]) {
             Ok(Expr::Literal {
                val: Some(Object::num(
                    self.previous().literal.unwrap().get_value().unwrap(),
                )),
            })
        } else if self.match_token(&[Token_type::STRING]) {
            Ok( Expr::Literal {
                val: Some(Object::str(
                    self.previous().literal.unwrap().get_value().unwrap(),
                )),
            })
        } else if self.match_token(&[Token_type::LEFT_PAREN]) {
             let expr=Expr::Grouping {
                expression: Box::from(self.expression()?),
            };
       self.cosume(&Token_type::RIGHT_PAREN, String::from("需要一个)"))?;
             Ok(expr)
        } else {
               Err(self.error(String::from("无效的表达式")))
        }
    }
    /*止*/
    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.pos += 1;
        }
        self.previous()
    }
    fn check(&self, token_type: &Token_type) -> bool {
        if self.is_end() {
            false
        } else {
            (*self.peek()).token_type == *token_type
        }
    }
    fn match_token(&mut self, types: &[Token_type]) -> bool {
        //当前的token 必须是需要的 才能继续
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }
    //判断错误
    fn cosume(&mut self,token_type: &Token_type,mes:String)->Result<Token,Parse_Err>{
        if self.check(&token_type) {
            Ok(self.advance())
        }else {
            Err(self.error(mes))
        }
    }
    fn is_end(&self) -> bool {
        self.previous().token_type == Token_type::EOF
    }
    //当前token
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    //前一个token
    fn previous(&self) -> Token {
        self.tokens[self.pos - 1].clone()
    }
    fn error(&mut self,mes:String)->Parse_Err{
        let token=self.peek();
        error::log(token.line,&token.lexeme,&mes);
         Parse_Err::new(token.clone(),mes)
    }
}
