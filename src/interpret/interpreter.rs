use crate::ast::expression::expr::{Expr};
use crate::ast::expression::expr;
use crate::ast::statment::stmt::{Stmt};
use crate::ast::statment::stmt;
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type;
use crate::error;
use crate::interpret::error::Run_Err;
use crate::interpret::value::Value;
use crate::tools::printf;
use crate::interpret::env::Environment;
use crate::{env_add,env_get};
pub(crate) struct Interpreter {}
impl expr::Visitor<Result<Value, Run_Err>> for Interpreter {
    fn visit_binary(
        &mut self,
        operator: &Token,
        l_expression: &Expr,
        r_expression: &Expr,
    ) -> Result<Value, Run_Err> {
        //判断是否出错
        let l = self.evaluate(l_expression).unwrap_or_else(|e| {
            error::log(e.token.line, &e.token.lexeme, &e.mes);
            Value::nil
        });
        let r = self.evaluate(r_expression).unwrap_or_else(|e| {
            error::log(e.token.line, &e.token.lexeme, &e.mes);
            Value::nil
        });
        match operator.token_type {
            Token_Type::PLUS => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(l + r)
            }
            Token_Type::MINUS => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(l + r)
            }
            Token_Type::STAR => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(l * r)
            }
            Token_Type::SLASH => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(l / r)
            }
            Token_Type::GREATER => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::bool(l > r))
            }
            Token_Type::LESS => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::bool(l < r))
            }
            Token_Type::GREATER_EQUAL => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::bool(l >= r))
            }
            Token_Type::EQUAL_EQUAL => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::bool(l == r))
            }
            Token_Type::BANG_EQUAL => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::bool(l != r))
            }
            Token_Type::LESS_EQUAL => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::bool(l <= r))
            }
            _ => Err(Run_Err::new(operator.clone(), String::from("操作符错误"))),
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<Value, Run_Err> {
        self.evaluate(expr)
    }

    fn visit_literal(&mut self, value: &Object) -> Result<Value, Run_Err> {
        Ok(match value {
            Object::str(s) => Value::str(s.clone()),
            Object::num(n) => Value::num(*n),
            Object::bool(b) => Value::bool(*b),
            _ => Value::nil,
        })
    }
    fn visit_unary(&mut self, operator: &Token, r_expression: &Expr) -> Result<Value, Run_Err> {
        let r_value = self.evaluate(r_expression);
        match operator.token_type {
            Token_Type::MINUS => Ok(-r_value?),
            Token_Type::BANG => Ok(!r_value?),
            _ => Err(Run_Err::new(operator.clone(), String::from("操作符错误"))),
        }
    }
    fn visit_variable(&mut self, name: &Token) -> Result<Value, Run_Err> {
        env_get!(name)
    }
}
impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {}
    }
    /*
    这里改成执行语句vec
    */
    pub(crate) fn run(&mut self, stmts:Vec<Stmt>) {
       for stmt in stmts {
           self.execute(stmt);
       }

    }
    fn execute(&mut self,stmt:Stmt){
         stmt.accept(self)
    }
    pub(crate) fn interpret(&mut self, expr: &Expr) -> Value {
        let res = self.evaluate(expr);
        res.unwrap_or_else(|e| {
            error::log(e.token.line, &e.token.lexeme, &e.mes);
            Value::nil
        })
    }
    pub(crate) fn check_num_operands(
        &self,
        oper: &Token,
        l: &Value,
        r: &Value,
    ) -> Result<(), Run_Err> {
        //加法操作支持str+num
        if oper.token_type == Token_Type::AND
            && ((l.is_num() && r.is_num()) || l.is_str() && r.is_num())
        {
            Ok(())
        } else if l.is_num() && r.is_num() {
            Ok(())
        } else {
            Err(Run_Err::new(
                oper.clone(),
                format!("此类型不支持{}操作", oper.lexeme),
            ))
        }
    }
    pub(crate) fn evaluate(&mut self, expr: &Expr) -> Result<Value, Run_Err> {
        expr.accept(self)
    }
    //这里其实可以复写
}
impl stmt::Visitor<Result<(),Run_Err>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr)->Result<(),Run_Err>{
       self.evaluate(expr);
        Ok(())
    }
    fn visit_print(&mut self, expr: &Expr)->Result<(),Run_Err>{
            let res= self.evaluate(expr);
          printf(res?);
        Ok(())

    }
    fn visit_let(&mut self,name:&Token,expr:&Expr)->Result<(),Run_Err>{
           //添加到变量池中
        let res=self.evaluate(expr);
        if let Ok(val) = res {
            env_add!(name,val)
        }else{
              Err(Run_Err::new(name.clone(), String::from("变量声明错误")))
        }
    }
}
//执行
