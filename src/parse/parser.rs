use std::collections::HashMap;
use crate::ast::expression::expr::Expr;
use crate::ast::statment::stmt::Stmt;
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type;
use crate::error::X_Err;
use crate::interpret::call::Fn_Type;
use crate::interpret::prototype::Prototype;
use crate::parse::Parse_Err;

#[derive(Debug)]
pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    //循环的深度
    loop_depth: usize,
}
impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            loop_depth: 0,
        }
    }
    fn assign_stmt(&mut self) -> Result<Expr, X_Err> {
        let expr = self.ternary_expr();
        if self.match_token(&[Token_Type::EQUAL]) {
            let _ = self.previous();
            let  val= self.assign_stmt()?;
            return if let Expr::Variable { name } = expr?.clone() {
                Ok(Expr::Assign {
                    name,
                    val: Box::from(val),
                })
            } else {
                Err(self.error(String::from("无效声明")))
            };
        }
        expr
    }
    fn and(&mut self) -> Result<Expr, X_Err> {
        let l = self.equality()?;
        while self.match_token(&[Token_Type::AND]) {
            let oper = self.previous();
            let r = self.equality()?;
            return Ok(Expr::Logical {
                operator: oper,
                l_expression: Box::from(l.clone()),
                r_expression: Box::from(r),
            });
        }
        Ok(l)
    }
    /*止*/
    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.pos += 1;
        }
        self.previous()
    }

    //添加{}中的语句
    fn block(&mut self) -> Result<Vec<Stmt>, X_Err> {
        let mut res = vec![];
        while !(self.is_end() || self.check(&Token_Type::RIGHT_BRACE)) {
            res.push(self.declaration()?);
        }
        self.consume(&Token_Type::RIGHT_BRACE, "此处缺少}")?;
        Ok(res)
    }
    fn break_stmt(&mut self) -> Result<Stmt, X_Err> {
        if self.loop_depth == 0 {
            Err(self.error(String::from("break只能在函数中使用")))
        } else {
            self.consume(&Token_Type::SEMICOLON, "break后须加分号")?;
            Ok(Stmt::Break {})
        }
    }
    fn continue_stmt(&mut self) -> Result<Stmt, X_Err> {
        if self.loop_depth == 0 {
            Err(self.error(String::from("continue只能在函数中使用")))
        } else {
            self.consume(&Token_Type::SEMICOLON, "continue后须加分号")?;
            Ok(Stmt::Continue {})
        }
    }
    //函数相关调用的代码
    fn call(&mut self) -> Result<Expr, X_Err> {
        let mut expr = self.primary()?;
        loop {
            if self.check(&Token_Type::LEFT_PAREN) {
                expr = self.finish_call(Box::from(expr))?;
            } else if self.check(&Token_Type::DOT) {
                let object = Box::from(expr.clone());
                let name = self.consume(&Token_Type::IDENTIFIER, "属性名")?;
                expr = Expr::Get { object, name }
            } else {
                break;
            }
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr, X_Err> {
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
    //判断错误
    fn consume(&mut self, token_type: &Token_Type, mes: &str) -> Result<Token, X_Err> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            //应该报错
            Err(self.error(String::from(mes)))
        }
    }
    fn check(&self, token_type: &Token_Type) -> bool {
        if self.is_end() {
            false
        } else {
            (*self.peek()).token_type == *token_type
        }
    }
    fn check_next(&self, token_type: &Token_Type) -> bool {
        if self.is_end() || self.tokens[self.pos + 1].token_type == Token_Type::EOF {
            false
        } else {
            self.tokens[self.pos + 1].token_type == *token_type
        }
    }
    fn declaration(&mut self) -> Result<Stmt, X_Err> {
        if self.match_token(&[Token_Type::STRUCT]) {
            self.struct_decl()
        } else if self.check(&Token_Type::FN) && self.check_next(&Token_Type::IDENTIFIER) {
            self.consume(&Token_Type::FN, "此处应该是个函数")?;
            self.func_stmt(Fn_Type::Func)
        } else if self.match_token(&[Token_Type::LET]) {
            self.let_declaration()
        } else {
            self.statement()
        }
    }
    /*
    执行单操作符
    */
    fn expression(&mut self) -> Expr {
        //先看看是不是三元
        if self.check(&Token_Type::IDENTIFIER)&&self.check_next(&Token_Type::LEFT_BRACE){

            self.prototype_expr().unwrap()}
        else if let Ok(expr) = self.assign_stmt() {
            println!("{:?}",self.tokens[self.pos]);
            expr
        }else{
            Expr::Literal { val: Object::Nil }
        }
    }

    //看看以等号为中心的表达式 类似 1!=2;1==2; 1==2&&2==1;
    fn equality(&mut self) -> Result<Expr, X_Err> {
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
    fn error(&mut self, mes: String) -> X_Err {
        let token = self.peek();
        Parse_Err::new(token.clone(), mes)
    }
    //下面是表达式转语句的代码
    fn expr_stmt(&mut self) -> Result<Stmt, X_Err> {
        let expr = self.assign_stmt()?;
        self.consume(&Token_Type::SEMICOLON, "表达式后需要分号")?;
        Ok(Stmt::Expression {
            expr: Box::new(expr),
        })
    }
    //是不是乘除
    fn factor(&mut self) -> Result<Expr, X_Err> {
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
    fn func_expr(&mut self, fn_type: Fn_Type) -> Result<Expr, X_Err> {
        // let name=self.consume(&Token_Type::IDENTIFIER,
        //      &("期望".to_owned()+fn_type.to_str()+"名字"))?;
        self.consume(&Token_Type::LEFT_PAREN, "期望(")?;
        let mut params = vec![];
        if !self.check(&Token_Type::RIGHT_PAREN) {
            loop {
                if params.len() > 255 {
                    self.error(String::from("参数数量不可以超过255个"));
                }
                params.push(self.consume(&Token_Type::IDENTIFIER, "期望一个参数名")?);
                if !self.match_token(&[Token_Type::COMMA]) {
                    break;
                }
            }
        }
        self.consume(&Token_Type::RIGHT_PAREN, "此处需要一个)")?;
        self.consume(&Token_Type::LEFT_BRACE, "此处需要一个{")?;
        //这里需要判断{吗？
        let body = self.block()?;
        Ok(Expr::Func { params, body })
    }
    fn func_stmt(&mut self, mut fn_type: Fn_Type) -> Result<Stmt, X_Err> {
        let name = self.consume(
            &Token_Type::IDENTIFIER,
            &("期望".to_owned() + fn_type.to_str() + "名字"),
        )?;

        Ok(Stmt::Func {
            name: Some(name),
            func: Box::from(self.func_expr(fn_type)?),
        })
    }
    fn for_stmt(&mut self) -> Result<Stmt, X_Err> {
        self.loop_depth += 1;
        //准备脱糖
        self.consume(&Token_Type::LEFT_PAREN, "此处应有一个(")?;
        let mut initializer: Option<Stmt> = None;
        if self.match_token(&[Token_Type::SEMICOLON]) {
            initializer = None;
        } else if self.match_token(&[Token_Type::LET]) {
            initializer = Some(self.let_declaration()?);
        } else {
            initializer = Some(self.expr_stmt()?);
            self.consume(&Token_Type::SEMICOLON, "此处应有一个;")?;
        };
        let mut condition: Expr = Expr::Literal {
            val: Object::Bool(true),
        };
        if !self.match_token(&[Token_Type::SEMICOLON]) {
            condition = self.expression();
        };
        self.consume(&Token_Type::SEMICOLON, "此处应有一个;")?;
        let mut increment = None;
        if !self.match_token(&[Token_Type::RIGHT_PAREN]) {
            increment = Some(self.expression());
        };
        self.consume(&Token_Type::RIGHT_PAREN, "此处应有一个)")?;
        let mut body = self.statement()?;
        //从后往前分类
        if increment.is_some() {
            body = Stmt::Block {
                stmts: vec![
                    body,
                    Stmt::Expression {
                        expr: Box::from(increment.unwrap()),
                    },
                ],
            };
        }
        body = Stmt::While {
            condition: Box::from(condition),
            body: Box::from(body),
        };
        if initializer.is_some() {
            body = Stmt::Block {
                stmts: vec![initializer.unwrap(), body],
            }
        }
        self.loop_depth -= 1;
        Ok(body)
    }
    fn finish_call(&mut self, callee: Box<Expr>) -> Result<Expr, X_Err> {
        let mut arguments = vec![];
        //    print!(printfself.tokens[self.pos]);
        self.advance();
        if !self.check(&Token_Type::RIGHT_PAREN) {
            loop {
                arguments.push(Box::from(self.expression()));
                //把参数添加进去 按逗号分割参数
                if !self.match_token(&[Token_Type::COMMA]) {
                    break;
                }
                if arguments.len() >= 255 {
                    return Err(self.error(String::from("参数数量最大为255")));
                }
            }
        };
        let paren = self.consume(&Token_Type::RIGHT_PAREN, "这里必须得有个)")?;
        Ok(Expr::Call {
            callee,
            paren,
            arguments,
        })
    }
    fn is_end(&self) -> bool {
        self.peek().token_type == Token_Type::EOF
    }

    fn if_stmt(&mut self) -> Result<Stmt, X_Err> {
        self.consume(&Token_Type::LEFT_PAREN, "此处缺少{")?; //先有个左括号
        let condition = Box::from(self.expression());
        self.consume(&Token_Type::RIGHT_PAREN, "此处缺少}")?; //再有个右括号
        let then_branch = Box::from(self.statement()?); //默认条件
        let mut else_branch = None;
        if self.match_token(&[Token_Type::ELSE]) {
            else_branch = Some(Box::from(self.statement()?));
        }
        Ok(Stmt::IF {
            condition,
            then_branch,
            else_branch,
        })
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

    fn or(&mut self) -> Result<Expr, X_Err> {
        let l = self.and()?;
        while self.match_token(&[Token_Type::OR]) {
            let oper = self.previous();
            let r = self.and()?;
            return Ok(Expr::Logical {
                operator: oper,
                l_expression: Box::from(l.clone()),
                r_expression: Box::from(r),
            });
        }
        Ok(l)
    }
    fn prototype_expr(&mut self) -> Result<Expr, X_Err> {
        //这里用来分析结构
        let name = self.consume(&Token_Type::IDENTIFIER, "此处缺少struct的名字")?;
        self.consume(&Token_Type::LEFT_BRACE, "此处缺少{")?;
        let mut keys =vec![];
        let mut vals=vec![];
        loop{
            if self.check(&Token_Type::RIGHT_BRACE){
                break;
            }
            let key=self.consume(&Token_Type::IDENTIFIER, "此处缺少字段名")?;
            self.consume(&Token_Type::COLON, "此处缺少:")?;
            println!("{:?}",self.tokens[self.pos]);
                keys.push(key);
               vals.push(self.primary()?);

        }
        //补充消费掉}
        self.consume(&Token_Type::RIGHT_BRACE,"")?;
        Ok(Expr::Instance { name, keys, vals } )
    }
    //非运算符的情况下
    //进行递归
    fn primary(&mut self) -> Result<Expr, X_Err> {
        if self.match_token(&[Token_Type::FN]) {
            Ok(self.func_expr(Fn_Type::Func)?)
        } else if self.match_token(&[Token_Type::NIL]) {
            Ok(Expr::Literal { val: Object::Nil })
        } else if self.match_token(&[Token_Type::TRUE]) {
            Ok(Expr::Literal {
                val: Object::Bool(true),
            })
        } else if self.match_token(&[Token_Type::FALSE]) {
            Ok(Expr::Literal {
                val: Object::Bool(false),
            })
        } else if self.match_token(&[Token_Type::NUMBER]) {
            Ok(Expr::Literal {
                val: self.previous().literal,
            })
        } else if self.match_token(&[Token_Type::STRING]) {
            Ok(Expr::Literal {
                val: self.previous().literal,
            })
        } else if self.match_token(&[Token_Type::IDENTIFIER]) {
            Ok(Expr::Variable {
                name: self.previous(),
            })
        } else if self.match_token(&[Token_Type::LEFT_PAREN]) {
            let expr = Expr::Grouping {
                expression: Box::from(self.expression()),
            };
            self.consume(&Token_Type::RIGHT_PAREN, "需要一个)")?;
            Ok(expr)
        } else {
            Err(self.error(String::from("无效的表达式")))
        }
    }
    pub(crate) fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while !self.is_end() {
            let stmt = self.declaration();
            if let Ok(stmt) = stmt {
                stmts.push(stmt);
            } else {
                self.advance();
            }
        }
        stmts
    }
    //当前token
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    //前一个token
    fn previous(&self) -> Token {
        self.tokens[self.pos - 1].clone()
    }
    fn print_stmt(&mut self) -> Result<Stmt, X_Err> {
        let val = self.expression();
        if let Err(e) = self.consume(&Token_Type::SEMICOLON, "此处应有分号") {
            Err(match e {
                X_Err::parse(parse_e) => self.error(parse_e.mes),
                _ => e,
            })
        } else {
            Ok(Stmt::Print {
                expr: Box::from(val),
            })
        }
    }
    fn return_stmt(&mut self) -> Result<Stmt, X_Err> {
        let keyword = self.previous();
        let expr = self.expression();
        self.consume(&Token_Type::SEMICOLON, "返回值也需要有分号结尾")?;
        Ok(Stmt::Return {
            keyword,
            expr: Box::from(expr),
        })
    }
    //声明语法
    //声明变量的规则
    fn let_declaration(&mut self) -> Result<Stmt, X_Err> {
        let name = self.consume(&Token_Type::IDENTIFIER, "这个单词不允许作为声明")?;
        let expr;
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
    fn statement(&mut self) -> Result<Stmt, X_Err> {
        if self.match_token(&[Token_Type::BREAK]) {
            self.break_stmt()
        } else if self.match_token(&[Token_Type::Continue]) {
            self.continue_stmt()
        } else if self.match_token(&[Token_Type::FOR]) {
            self.for_stmt()
        } else if self.match_token(&[Token_Type::IF]) {
            self.if_stmt()
        } else if self.match_token(&[Token_Type::PRINT]) {
            self.print_stmt()
        } else if self.match_token(&[Token_Type::RETURN]) {
            self.return_stmt()
        } else if self.match_token(&[Token_Type::WHILE]) {
            self.while_stmt()
        } else if self.match_token(&[Token_Type::LEFT_BRACE]) {
            Ok(Stmt::Block {
                stmts: self.block()?,
            })
        } else {
            self.expr_stmt()
        }
    }
    fn struct_decl(&mut self) -> Result<Stmt, X_Err> {
        let name = self.consume(&Token_Type::IDENTIFIER, "结构体需要命名")?;
        let mut fields = vec![];
        self.consume(&Token_Type::LEFT_BRACE, "结构体内容前需要{")?;
        while self.match_token(&[Token_Type::IDENTIFIER]) && !self.is_end() {
            fields.push(self.previous().clone());
            if self.check(&Token_Type::RIGHT_BRACE) {
                break;
            } else {
                self.consume(&Token_Type::COMMA, "属性需要用逗号隔开")?;
            }
        }
        self.consume(&Token_Type::RIGHT_BRACE, "结构体内容后需要}")?;
        self.consume(&Token_Type::SEMICOLON, "结构体声明后需要;")?;
        Ok(Stmt::Struct { name, fields })
    }
    //是不是加减
    fn term(&mut self) -> Result<Expr, X_Err> {
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
    //解析三元表达式
    fn ternary_expr(&mut self) -> Result<Expr, X_Err> {
        // 先解析条件表达式 以等号为核心
        let may_expr = self.or()?;
        //不是三元直接返回
        if !self.match_token(&[Token_Type::QUESTION]) {
            return Ok(may_expr);
        }
        // 解析 ? 后的表达式
        let t_expr = self.expression();
        // 必须有冒号
        self.consume(&Token_Type::COLON, "此处三元表达式缺少:'")?;
        let f_expr = self.expression();
        Ok(Expr::Ternary {
            condition: Box::new(may_expr),
            t_expr: Box::new(t_expr),
            f_expr: Box::new(f_expr),
        })
    }

    //
    fn unary(&mut self) -> Result<Expr, X_Err> {
        if self.match_token(&[Token_Type::BANG, Token_Type::MINUS]) {
            let operator = self.previous();
            let r_expression = Box::from(self.unary()?);
            return Ok(Expr::Unary {
                operator,
                r_expression,
            });
        }
        self.call()
    }

    fn while_stmt(&mut self) -> Result<Stmt, X_Err> {
        self.loop_depth += 1;
        self.consume(&Token_Type::LEFT_PAREN, "此处应有一个(")?;
        let expr = self.expression();
        self.consume(&Token_Type::RIGHT_PAREN, "此处应有一个)")?;
        let body = self.statement()?;
        self.loop_depth -= 1;
        Ok(Stmt::While {
            condition: Box::from(expr),
            body: Box::from(body),
        })
    }
}
//表达式解析的时候看看是不是三元
