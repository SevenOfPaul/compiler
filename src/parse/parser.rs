use crate::ast::expression::expr::Expr;
use crate::ast::statment::stmt::Stmt;
use crate::ast::token::object::{ Object};
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type;
use crate::parse::err::Parse_Err;

#[derive(Debug)]
pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    pub(crate) fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while !self.is_end() {
           let stmt=self.declaration();
            if let Ok(stmt) = stmt {
                stmts.push(stmt);
            }else{
                self.advance();
            }
        }
        stmts
    }
    /*
    执行单操作符
    */
    fn expression(&mut self) -> Expr {
        if let Ok(expr) = self.assign_stmt() {
            expr
        } else {
            Expr::Literal { val: Object::nil }
        }
    }

    fn declaration(&mut self) -> Result<Stmt, Parse_Err> {
        if self.match_token(&[Token_Type::LET]) {
            self.let_declaration()
        } else {
            self.statement()
        }
    }
    //声明变量的规则
    fn let_declaration(&mut self) -> Result<Stmt, Parse_Err> {
        let name = self.consume(
            &Token_Type::IDENTIFIER,
            "这个单词不允许作为声明",
        )?;
        let mut expr;
        if self.match_token(&[Token_Type::EQUAL]) {
            expr = self.expression();
            self.consume(&Token_Type::SEMICOLON, "此处应有分号结尾")?;
            Ok(Stmt::LET {
                name,
                expr: Box::new(expr),
            })
        } else {
            Err(self.error(String::from("此处声明出错")))
        }
    }
    //把表达式转为语句
    fn statement(&mut self) -> Result<Stmt, Parse_Err> {
        if self.match_token(&[Token_Type::IF]){
              self.if_stmt()
        }else if self.match_token(&[Token_Type::PRINT]) {
            self.print_stmt()
        }else if self.match_token(&[Token_Type::LEFT_BRACE]) {
            Ok(Stmt::Block {stmts:self.block()?})
        } else {
            self.expr_stmt()
        }
    }
    //看看等号的运算符
    fn equality(&mut self) -> Result<Expr, Parse_Err> {
        let mut expr = self.comparison();
        while self.match_token(&[Token_Type::BANG_EQUAL, Token_Type::EQUAL_EQUAL]) {
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
    fn comparison(&mut self) -> Result<Expr, Parse_Err> {
        let mut expr = self.term();
        while self.match_token(&[
            Token_Type::LESS,
            Token_Type::GREATER,
            Token_Type::LESS_EQUAL,
            Token_Type::GREATER_EQUAL,
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
    fn term(&mut self) -> Result<Expr, Parse_Err> {
        let mut expr = self.factor();
        while self.match_token(&[Token_Type::MINUS, Token_Type::PLUS]) {
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
    //添加{}中的语句
    fn block(&mut self) ->Result<Vec<Stmt>,Parse_Err>{
     let mut res=vec![];
        while !(self.is_end()||self.check(&Token_Type::RIGHT_BRACE)) {
            res.push(self.declaration()?);
        }
        self.consume(&Token_Type::RIGHT_BRACE,"此处缺少}");
        Ok(res)
    }
    //是不是乘除
    fn factor(&mut self) -> Result<Expr, Parse_Err> {
        let mut expr = self.unary();
        while self.match_token(&[Token_Type::SLASH, Token_Type::STAR]) {
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
    fn unary(&mut self) -> Result<Expr, Parse_Err> {
        if self.match_token(&[Token_Type::BANG, Token_Type::MINUS]) {
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
    fn primary(&mut self) -> Result<Expr, Parse_Err> {
        if self.match_token(&[Token_Type::NIL]) {
            Ok(Expr::Literal { val: Object::nil })
        } else if self.match_token(&[Token_Type::TRUE]) {
            Ok(Expr::Literal {
                val: Object::bool(true),
            })
        } else if self.match_token(&[Token_Type::FALSE]) {
            Ok(Expr::Literal {
                val: Object::bool(false),
            })
        } else if self.match_token(&[Token_Type::NUMBER]) {
            Ok(Expr::Literal {
                val: self.previous().literal.unwrap(),
            })
        } else if self.match_token(&[Token_Type::STRING]) {
            Ok(Expr::Literal {
                val: self.previous().literal.unwrap(),
            })
        }else if self.match_token(&[Token_Type::IDENTIFIER]) {
                  Ok(Expr::Variable{
                      name:self.previous(),
                  })
        } else if self.match_token(&[Token_Type::LEFT_PAREN]) {
            let expr = Expr::Grouping {
                expression: Box::from(self.expression()),
            };
            self.consume(&Token_Type::RIGHT_PAREN, "需要一个)")?;
            Ok(expr)
        }else{
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
    fn check(&self, token_type: &Token_Type) -> bool {
        if self.is_end() {
            false
        } else {
            (*self.peek()).token_type == *token_type
        }
    }
    fn match_token(&mut self, types: &[Token_Type]) -> bool {
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
    fn consume(&mut self, token_type: &Token_Type, mes: &str) -> Result<Token, Parse_Err> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            //应该报错
            Err(self.error(String::from(mes)))
        }
    }

    fn is_end(&self) -> bool {
        self.peek().token_type == Token_Type::EOF
    }
    //当前token
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    //前一个token
    fn previous(&self) -> Token {
        self.tokens[self.pos - 1].clone()
    }
    fn error(&mut self, mes: String) -> Parse_Err {
        let token = self.peek();
        Parse_Err::new(token.clone(),mes)
    }
    //下面是表达式转语句的代码
    fn expr_stmt(&mut self) -> Result<Stmt, Parse_Err> {
        let val = self.expression();
        if let Err(e) = self.consume(&Token_Type::SEMICOLON, "此处应有分号") {
            Err(self.error(e.mes))
        } else {
            Ok(Stmt::Expression {
                expr: Box::from(val),
            })
        }
    }
    fn if_stmt(&mut self)->Result<Stmt,Parse_Err>{
         self.consume(&Token_Type::LEFT_PAREN,"此处缺少{")?;//先有个左括号
         let condition=Box::from(self.expression());
         self.consume(&Token_Type::RIGHT_PAREN, "此处缺少}")?;//再有个右括号
          let then_branch=Box::from(self.statement()?);//默认条件
         let mut else_branch=None;
          if self.match_token(&[Token_Type::ELSE]){
             else_branch=Some(Box::from(self.statement()?));
        }
          Ok(Stmt::IF {condition,then_branch,else_branch})
    }
    fn assign_stmt(&mut self)->Result<Expr,Parse_Err>{
        let expr=self.equality();
        if self.match_token(&[Token_Type::EQUAL]) {
            let equals = self.previous();
            let val=self.assign_stmt()?;
            return  if let Expr::Variable {name} = expr?.clone() {
                   Ok(Expr::Assign { name, val: Box::from(val) })
            }else{
                 Err(self.error(String::from("无效声明")))
            }
        }
         expr
    }
    // fn block_stmt(&mut self)->Vec<Stmt>{
    //     let mut stmts =vec![];
    //     while !(self.match_token(&[Token_Type::LEFT_BRACE])||self.is_end()) {
    //         self.declaration().and_then(|stmt| Ok(stmts.push(stmt)));
    //     }
    //     stmts
    // }
    fn print_stmt(&mut self) -> Result<Stmt, Parse_Err> {
        let val = self.expression();
        if let Err(e) = self.consume(&Token_Type::SEMICOLON, "此处应有分号") {
            Err(self.error(e.mes))
        } else {
            Ok(Stmt::Print {
                expr: Box::from(val)
            })
        }
    }
    //声明语法
}
