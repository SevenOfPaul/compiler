use crate::ast::expression::expr;
use crate::ast::expression::expr::Expr;
use crate::ast::statment::stmt;
use crate::ast::statment::stmt::Stmt;
use crate::ast::token::object::Object;
use crate::ast::token::token::Token;
use crate::ast::token::token_type::Token_Type;
use crate::error;
use crate::error::X_Err;
use crate::interpret::call::Funcs;
use crate::interpret::call::{Call, Fn_init, Func};
use crate::interpret::env::Environment;
use crate::interpret::prototype::Property;
use crate::interpret::value::Value;
use crate::interpret::{Return, Run_Err};
use crate::parse::{Break, Continue};
use crate::tools::printf;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::prototype::Prototype;
/**只能有一个 */
pub(crate) struct Interpreter {
    pub(crate) env: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, i32>,
}
impl expr::Visitor<Result<Value, X_Err>> for Interpreter {
    fn visitor_assign(
        &mut self,
        expr: &Expr,
        name: &Token,
        value: &Box<Expr>,
    ) -> Result<Value, X_Err> {
        let val = self.evaluate(value)?;
        let distance = self.locals.get(expr);
        if let Some(d) = distance {
            self.env.borrow_mut().assign_at(*d, name, val)
        } else {
            self.env.borrow_mut().assign(name, val)
        }
    }
    fn visitor_binary(
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
    fn visitor_call(
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
    fn visitor_instance(
        &mut self,
        struct_name: &Box<Expr>,
        keys: &Vec<Token>,
        vals: &Vec<Expr>,
    ) -> Result<Value, X_Err> {
        let vals = &vals.iter().map(|v| self.evaluate(v)).collect::<Result<Vec<Value>, X_Err>>()?;
        Ok(Value::Instance(Prototype::new(
            struct_name.clone(),keys,vals
        )))
    }
    fn visitor_func(&mut self, params: &Vec<Token>, body: &Vec<Stmt>) -> Result<Value, X_Err> {
        //这里有问题
        //得大改 周末吃透
        //这个是匿名函数表达式
        let func_stmt = Stmt::Func {
            name: None,
            func: Box::from(Expr::Func {
                params: params.clone(),
                body: body.clone(),
            }),
        };
        Ok(Value::Func(Func::new(func_stmt)))
    }
    fn visitor_get(&mut self, object: &Expr, name: &Token) -> Result<Value, X_Err> {
        let prototype = self.evaluate(object)?;
        if let Value::Instance(obj) = prototype {
            Ok(obj.get(name))
        } else {
            Err(Run_Err::new(name.clone(), String::from("不是一个结构体")))
        }
    }
    fn visitor_grouping(&mut self, expr: &Expr) -> Result<Value, X_Err> {
        self.evaluate(expr)
    }

    fn visitor_literal(&mut self, value: &Object) -> Result<Value, X_Err> {
        Ok(match value {
            Object::Str(s) => Value::Str(s.clone()),
            Object::Num(n) => Value::Num(*n),
            Object::Bool(b) => Value::Bool(*b),
            _ => Value::Nil,
        })
    }
    fn visitor_logical(
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
    ///设置对象的属性
    fn visitor_set(&mut self, object: &Expr, name: &Token, val: &Box<Expr>) -> Result<Value, X_Err> {
        //这里因为需要获取作用于中唯一的数据 所以使用额外的辅助函数
        if let Expr::Variable { name: var_name } = object {
            // 获取变量引用
            let rc_val = self.get_variable_ref(object, var_name)?;
            let val = self.evaluate(val)?;
            // 修改引用指向的值
            let mut value = rc_val.borrow_mut();
            if let Value::Instance(ref mut instance) = *value {
                instance.set(name, val.clone());
                Ok(val)
            } else {
                Err(Run_Err::new(name.clone(), String::from("不是一个结构体")))
            }
        } else {
            Err(Run_Err::new(name.clone(), String::from("无效的赋值目标")))
        }
    }
        ///在作用于中声明class
    fn visitor_struct(&mut self, name: &Token, _fields: &Vec<Token>) -> Result<Value, X_Err> {
        self.env.borrow_mut().add(name, Value::Nil)?;
        //这里需要真正声明class
        let r#struct = Value::Struct { name: name.clone() };
        self.env.borrow_mut().assign(name, r#struct.clone())?;
        Ok(r#struct)
    }
    fn visitor_ternary(
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
    fn visitor_unary(&mut self, operator: &Token, r_expression: &Expr) -> Result<Value, X_Err> {
        let r_value = self.evaluate(r_expression);
        match operator.token_type {
            Token_Type::MINUS => Ok(-r_value?),
            Token_Type::BANG => Ok(!r_value?),
            _ => Err(Run_Err::new(operator.clone(), String::from("操作符错误"))),
        }
    }
    fn visitor_variable(&mut self, expr: &Expr, name: &Token) -> Result<Value, X_Err> {
        // 获取引用
        let rc_val = self.lookup_variable(expr, name)?;
        // 克隆值并存储在新变量中
        let value = rc_val.borrow().clone();
        Ok(value)
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
            locals: HashMap::new(),
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
        } else if l.is_time() && r.is_time() {
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
    pub(crate) fn resolve(&mut self, expr: &Expr, depth: i32) -> Result<(), X_Err> {
        self.locals.insert(expr.clone(), depth);
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
    ///查找变量
    pub(crate) fn lookup_variable(&self, expr: &Expr, name: &Token) -> Result<Rc<RefCell<Value>>, X_Err> {
        if let Some(&distance) = self.locals.get(expr) {
            self.env.borrow_mut().get_at(distance, name) // 本地查找
        } else {
            //这里没有降级到全局查找
            self.env.borrow_mut().get_by_global(name) // 降级到全局查找
        }
    }
    pub(crate) fn get_variable_ref(&self, expr: &Expr, name: &Token) -> Result<Rc<RefCell<Value>>, X_Err> {
        self.lookup_variable(expr, name)
    }
    // 新增帮助函数,用于需要获取引用的场景
}
impl stmt::Visitor<Result<(), X_Err>> for Interpreter {
    fn visitor_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), X_Err> {
        //这里要支持嵌套
        //这里得改
        self.execute_block(stmts, Environment::new(Some(self.env.clone())))?;
        Ok(())
    }
    fn visitor_break(&mut self) -> Result<(), X_Err> {
        Err(Break::new())
    }
    fn visitor_continue(&mut self) -> Result<(), X_Err> {
        Err(Continue::new())
    }
    fn visitor_expr(&mut self, expr: &Expr) -> Result<(), X_Err> {
        self.evaluate(expr)?;
        Ok(())
    }
    fn visitor_func(
        &mut self,
        _: &Stmt,
        name: &Option<Token>,
        func: &Box<Expr>,
    ) -> Result<(), X_Err> {
        //这里不明白
        let func = Stmt::Func {
            name: name.clone(),
            func: func.clone(),
        };
        self.env
            .borrow_mut()
            .add(&(name.clone().unwrap()), Value::Func(Func::new(func)))?;
        Ok(())
    }
    fn visitor_if(
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

    fn visitor_let(&mut self, name: &Token, expr: &Expr) -> Result<(), X_Err> {
        //添加到变量池中
        let res = self.evaluate(expr);
        if let Ok(val) = res {
            self.env.borrow_mut().add(name, val)?;
            Ok(())
        } else {
            Err(Run_Err::new(name.clone(), String::from("变量声明错误")))
        }
    }
    fn visitor_print(&mut self, expr: &Expr) -> Result<(), X_Err> {
        printf(self.evaluate(expr)?);
        Ok(())
    }
    fn visitor_return(&mut self, keyword: &Token, expr: &Expr) -> Result<(), X_Err> {
        let val = self.evaluate(expr)?;
        Err(Return::new(val))
    }
    fn visitor_while(&mut self, condition: &Expr, body: &Stmt) -> Result<(), X_Err> {
        while self.evaluate(condition)?.is_truthy() {
            //借用and实现
            let res = self.execute(body.clone());
            if let Err(x) = res {
                match x {
                    X_Err::brk(r#break) => break,
                    X_Err::cte(r#continue) => {
                        //continue暂时有问题
                        continue;
                    }
                    _ => return Err(x),
                }
            }
        }
        Ok(())
    }

    fn visitor_impl(&mut self, prototype: &Token, methods: &Vec<Stmt>) -> Result<(), X_Err> {
        //需要修改
        todo!()
    }
}
