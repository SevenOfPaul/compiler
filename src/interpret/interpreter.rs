use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::expression::expr::Expr;
use crate::ast::expression::expr;
use crate::ast::statment::stmt::Stmt;
use crate::ast::statment::stmt;
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type;
use crate::call::{Call, Funcs};
use crate::error;
use crate::interpret::error::Run_Err;
use crate::interpret::value::Value;
use crate::tools::printf;
use crate::interpret::env::Environment;
pub(crate) struct Interpreter {
    env: Rc<RefCell<Environment>>,
}
impl expr::Visitor<Result<Value, Run_Err>> for Interpreter {
    fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> Result<Value, Run_Err> {
        let val = self.evaluate(value)?;
        self.env.borrow_mut().set(name, val)
    }
    fn visit_binary(
        &mut self,
        operator: &Token,
        l_expression: &Expr,
        r_expression: &Expr,
    ) -> Result<Value, Run_Err> {
        //判断是否出错
        let l = self.evaluate(l_expression).unwrap_or_else(|e| {
            error::log(e.token.line, &e.token.lexeme, &e.mes);
            Value::Nil
        });
        let r = self.evaluate(r_expression).unwrap_or_else(|e| {
            error::log(e.token.line, &e.token.lexeme, &e.mes);
            Value::Nil
        });
        match operator.token_type {
            Token_Type::PLUS => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(l + r)
            }
            Token_Type::MINUS => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(l - r)
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
                Ok(Value::Bool(l > r))
            }
            Token_Type::LESS => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::Bool(l < r))
            }
            Token_Type::GREATER_EQUAL => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::Bool(l >= r))
            }
            Token_Type::EQUAL_EQUAL => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::Bool(l == r))
            }
            Token_Type::BANG_EQUAL => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::Bool(l != r))
            }
            Token_Type::LESS_EQUAL => {
                self.check_num_operands(operator, &l, &r)?;
                Ok(Value::Bool(l <= r))
            }
            _ => Err(Run_Err::new(operator.clone(), String::from("操作符错误"))),
        }
    }
    fn visit_call(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Box<Expr>>) -> Result<Value, Run_Err> {
      let  expr=self.evaluate(callee)?;
      if arguments.len()!=expr.arity(){
         return Err(Run_Err::new(paren.clone(), String::from("参数数量不符合调用要求")));
      }
        //感觉这里有问题
        let mut arguments_func =vec![];
        for argu in arguments {
            arguments_func.push(self.evaluate(argu)?);
        }
       Ok(expr.call(arguments_func))
    }
    fn visit_grouping(&mut self, expr: &Expr) -> Result<Value, Run_Err> {
        self.evaluate(expr)
    }

    fn visit_literal(&mut self, value: &Object) -> Result<Value, Run_Err> {
        Ok(match value {
            Object::Str(s) => Value::Str(s.clone()),
            Object::Num(n) => Value::Num(*n),
            Object::Bool(b) => Value::Bool(*b),
            _ => Value::Nil,
        })
    }
    fn visit_logical(
        &mut self,
        operator: &Token,
        l_expression: &Box<Expr>,
        r_expression: &Box<Expr>,
    ) -> Result<Value, Run_Err> {
        let l = self.evaluate(l_expression)?;
        if operator.token_type == Token_Type::OR {
            if l.is_truthy() {
                return Ok(l);
            }
        } else {
            if !l.is_truthy() {
                return Ok(l);
            }
        }
        Ok(self.evaluate(r_expression)?)
    }
    fn visit_ternary(
        &mut self,
        condition: &Box<Expr>,
        t_expr: &Box<Expr>,
        f_expr: &Box<Expr>,
    ) -> Result<Value, Run_Err> {
        Ok(if self.evaluate(condition)?.is_truthy() {
            self.evaluate(t_expr)?
        } else {
            self.evaluate(f_expr)?
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
        self.env.borrow().get(name)
    }
}
impl Interpreter {
    pub(crate) fn new() -> Self {
        let  mut global=Environment::new(None);
        //读取所有原生函数 添加到最高作用域中
           global.init_fn(Funcs.keys().collect::<Vec<&String>>());
        //最高作用域
        Self {
            env: Rc::from(RefCell::from(global)),
        }
    }
    pub(crate) fn check_num_operands(
        &self,
        oper: &Token,
        l: &Value,
        r: &Value,
    ) -> Result<(), Run_Err> {
        //加法操作支持str+num
        if oper.token_type == Token_Type::PLUS
            && ((l.is_num() && r.is_num()) || l.is_str() && r.is_num())
        {
            Ok(())
        } else if l.is_num() && r.is_num() {
            Ok(())
        } else {
            error::log(
                oper.line,
                &oper.lexeme,
                &format!("此类型不支持{}操作", oper.lexeme),
            );
            Err(Run_Err::new(
                oper.clone(),
                format!("此类型不支持{}操作", oper.lexeme),
            ))
        }
    }
    fn execute(&mut self, stmt: Stmt) {
        stmt.accept(self)
    }
    pub(crate) fn evaluate(&mut self, expr: &Expr) -> Result<Value, Run_Err> {
        expr.accept(self)
    }
    fn execute_block(&mut self, stmts: &Vec<Stmt>, env: Environment) -> Result<(), Run_Err> {
        let pre_env = self.env.clone();
        self.env = Rc::new(RefCell::new(env));
        for stmt in stmts {
            self.execute(stmt.clone());
        }
        self.env = pre_env;
        Ok(())
    }
    /*
    这里改成执行语句vec
    */
    pub(crate) fn run(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.execute(stmt);
        }
    }

    //这里其实可以复写
}
impl stmt::Visitor<Result<(), Run_Err>> for Interpreter {
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), Run_Err> {
        //这里要支持嵌套
        //这里得改
        self.execute_block(stmts, Environment::new(Some(self.env.clone())))?;
        Ok(())
    }
    fn visit_expr(&mut self, expr: &Expr) -> Result<(), Run_Err> {
        self.evaluate(expr)?;
        Ok(())
    }
    fn visit_let(&mut self, name: &Token, expr: &Expr) -> Result<(), Run_Err> {
        //添加到变量池中
        let res = self.evaluate(expr);
        if let Ok(val) = res {
            self.env.borrow_mut().add(name, val)
        } else {
            Err(Run_Err::new(name.clone(), String::from("变量声明错误")))
        }
    }
    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<(), Run_Err> {
        if self.evaluate(condition)?.is_truthy() {
            self.execute(then_branch.clone());
        } else if else_branch.is_some() {
            self.execute(else_branch.unwrap().clone());
        }
        Ok(())
    }
    fn visit_print(&mut self, expr: &Expr) -> Result<(), Run_Err> {
        printf(self.evaluate(expr)?);
        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Result<(), Run_Err> {
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body.clone());
        }
        Ok(())
    }
}
//执行
