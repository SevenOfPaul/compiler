use crate::ast::expression::expr;
use crate::ast::expression::expr::Expr;
use crate::ast::statment::stmt;
use crate::ast::statment::stmt::Stmt;
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type;
use crate::interpret::call::Funcs;
use crate::interpret::call::{Call, Fn_init, Func};
use crate::error;
use crate::error::X_Err;
use crate::interpret::env::Environment;
use crate::interpret::value::Value;
use crate::interpret::{Return, Run_Err};
use crate::parse::{Break, Continue};
use crate::tools::printf;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Clone)]
pub(crate) struct Interpreter {
    pub(crate) env: Rc<RefCell<Environment>>,
    locals:HashMap<Expr,i32>
}
impl expr::Visitor<Result<Value, X_Err>> for Interpreter {
    fn visit_assign(&mut self,expr:&Expr,name: &Token, value: &Box<Expr>) -> Result<Value, X_Err> {
        let val = self.evaluate(value)?;
        let distance=self.locals.get(expr);
        return  if let Some(d)=distance{
            self.env.borrow_mut().assign_at(*d, name, val)
        }else{
            self.env.borrow_mut().set(name, val)
        }
    }
    fn visit_binary(
        &mut self,
        operator: &Token,
        l_expression: &Expr,
        r_expression: &Expr,
    ) -> Result<Value, X_Err> {
        //判断是否出错
        let l = self.evaluate(l_expression).unwrap_or_else(|e| {
            if let X_Err::run(run_err) = e {
                error::log(run_err.token.line, &run_err.token.lexeme, &run_err.mes);
            }
            Value::Nil
        });
        let r = self.evaluate(r_expression).unwrap_or_else(|e| {
            if let X_Err::run(run_err) = e {
                error::log(run_err.token.line, &run_err.token.lexeme, &run_err.mes);
            }
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
    fn visit_call(
        &mut self,
        callee: &Box<Expr>,
        paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> Result<Value, X_Err> {
        let expr = self.evaluate(callee)?;
        if arguments.len() != expr.arity() {
            return Err(Run_Err::new(
                paren.clone(),
                String::from("参数数量不符合调用要求"),
            ));
        }
        //感觉这里有问题
        let mut arguments_func = vec![];
        for argu in arguments {
            arguments_func.push(self.evaluate(argu)?);
        }
        expr.call(self, arguments_func)
    }
    fn visit_func(&mut self,params:&Vec<Token>,body:&Vec<Stmt>)->Result<Value, X_Err> {
        //这里有问题
        //得大改 周末吃透
        let func_stmt=Stmt::Func { name: None, func: Box::from(Expr::Func { params:params.clone(), body: body.clone() }) };
        Ok(Value::Func(Func::new(func_stmt)))
    }
    fn visit_grouping(&mut self, expr: &Expr) -> Result<Value, X_Err> {
        self.evaluate(expr)
    }

    fn visit_literal(&mut self, value: &Object) -> Result<Value, X_Err> {
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
    ) -> Result<Value, X_Err> {
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
    ) -> Result<Value, X_Err> {
        Ok(if self.evaluate(condition)?.is_truthy() {
            self.evaluate(t_expr)?
        } else {
            self.evaluate(f_expr)?
        })
    }
    fn visit_unary(&mut self, operator: &Token, r_expression: &Expr) -> Result<Value, X_Err> {
        let r_value = self.evaluate(r_expression);
        match operator.token_type {
            Token_Type::MINUS => Ok(-r_value?),
            Token_Type::BANG => Ok(!r_value?),
            _ => Err(Run_Err::new(operator.clone(), String::from("操作符错误"))),
        }
    }
    fn visit_variable(&mut self,expr:&Expr, name: &Token) -> Result<Value, X_Err> {
        self.lookup_variable(expr,name)
        // self.env.borrow().get(name)
    }
}
impl Interpreter {
    pub(crate) fn new() -> Self {
        let mut global = Environment::new(None);
        //读取所有原生函数 添加到最高作用域中
        global.init_fn(Funcs.keys().collect::<Vec<&String>>());
        //最高作用域
        Self {
            env: Rc::from(RefCell::from(global)),
            //需要修改
            locals:HashMap::new()
        }
    }
    pub(crate) fn check_num_operands(
        &self,
        oper: &Token,
        l: &Value,
        r: &Value,
    ) -> Result<(), X_Err> {
        //加法操作支持str+num
        if oper.token_type == Token_Type::PLUS
            && ((l.is_num() && r.is_num()) || l.is_str() && r.is_num())
        {
            Ok(())
        } else if l.is_str() && r.is_str() {
            Ok(())
        } else if l.is_num() && r.is_num() {
            Ok(())
        }else if l.is_time()&&r.is_time(){
            Ok(())
        } else {
            Err(Run_Err::new(
                oper.clone(),
                format!("此类型不支持{}操作", oper.lexeme),
            ))
        }
    }
    fn execute(&mut self, stmt: Stmt) -> Result<(), X_Err> {
        stmt.accept(self)
    }
    pub(crate) fn evaluate(&mut self, expr: &Expr) -> Result<Value, X_Err> {
        expr.accept(self)
    }
    pub(crate) fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        env: Environment,
    ) -> Result<(), X_Err> {
        let pre_env = self.env.clone();
        self.env = Rc::new(RefCell::new(env));
        for stmt in stmts {
            self.execute(stmt.clone())?;
        }
        self.env = pre_env;
        Ok(())
    }
    pub(crate) fn resolve(&mut self,expr:&Expr,depth:i32)->Result<(),X_Err>{
        println!("Interpreter resolving: depth={}", depth);  // 添加调试信息
        self.locals.insert(expr.clone(),depth);
        println!("Locals after insert: {:?}", self.locals);
        Ok(())
    }
    /*
    这里改成执行语句vec
    */
    pub(crate) fn run(&mut self, stmts: &Vec<Stmt>) -> Result<(), X_Err> {
        for stmt in stmts {
            self.execute(stmt.clone())?;
        }
        Ok(())
    }

    pub (crate) fn lookup_variable(&self,expr:&Expr,name:&Token)->Result<Value,X_Err>{
        // let distance=self.locals.get(&expr);
        if let Some(&distance) = self.locals.get(expr) {
            self.env.borrow_mut().get_at(distance,name)  // 全局查找
        } else {
            self.env.borrow().get(name)  // 降级到全局查找
        }
    }
}
impl stmt::Visitor<Result<(), X_Err>> for Interpreter {
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), X_Err> {
        //这里要支持嵌套
        //这里得改
        self.execute_block(stmts, Environment::new(Some(self.env.clone())))?;
        Ok(())
    }
    fn visit_break(&mut self) -> Result<(), X_Err> {
        Err(Break::new())
    }
    fn visit_continue(&mut self) -> Result<(), X_Err> {
        Err(Continue::new())
    }
    fn visit_expr(&mut self, expr: &Expr) -> Result<(), X_Err> {
        self.evaluate(expr)?;
        Ok(())
    }
    fn visit_func(
        &mut self,_:&Stmt,
        name: &Option<Token>,
        func:&Box<Expr>
    ) -> Result<(), X_Err> {
        //这里不明白
        let func = Stmt::Func {
            name:name.clone(),func: func.clone()
        };
        self.env
            .borrow_mut()
            .add(&(name.clone().unwrap()), Value::Func(Func::new(func)))?;
        Ok(())
    }
    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<(), X_Err> {
        if self.evaluate(condition)?.is_truthy() {
            self.execute(then_branch.clone())?
        } else if else_branch.is_some() {
            self.execute(else_branch.unwrap().clone())?
        }
        Ok(())
    }
    fn visit_let(&mut self, name: &Token, expr: &Expr) -> Result<(), X_Err> {
        //添加到变量池中
        let res = self.evaluate(expr);
        if let Ok(val) = res {
            self.env.borrow_mut().add(name, val)?;
            Ok(())
        } else {
            Err(Run_Err::new(name.clone(), String::from("变量声明错误")))
        }
    }
    fn visit_print(&mut self, expr: &Expr) -> Result<(), X_Err> {
        printf(self.evaluate(expr)?);
        Ok(())
    }
    fn visit_return(&mut self, keyword: &Token, expr: &Expr) -> Result<(), X_Err> {
        let val = self.evaluate(expr)?;
        Err(Return::new(val))
    }
    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Result<(), X_Err> {
        while self.evaluate(condition)?.is_truthy() {
            //借用and实现
            let res = self.execute(body.clone());
            if let Err(x) = res {
                match x {
                    X_Err::brk(Break) => break,
                    X_Err::cte(Continue) => {
                        //continue暂时有问题
                        continue;
                    }
                    _ => return Err(x),
                }
            }
        }
        Ok(())
    }
}
//执行
