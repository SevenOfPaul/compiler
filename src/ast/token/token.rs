use crate::ast::token::object::Object;
use crate::ast::token::token_type::Token_Type;
use lazy_static::lazy_static;
use std::collections::HashMap;
#[derive(Clone, Debug,PartialEq, Eq,Hash)]
pub(crate) struct Token {
    pub(crate) token_type: Token_Type,
    pub(crate) lexeme: String,
    pub(crate) literal: Object,
    pub(crate) line: usize,
}
lazy_static! {
    pub(crate) static ref Keywords: HashMap<String, Token_Type> = {
        HashMap::from([
            (String::from("break"), Token_Type::BREAK),
            (String::from("continue"), Token_Type::Continue),
            (String::from("else"), Token_Type::ELSE),
            (String::from("false"), Token_Type::FALSE),
            (String::from("fn"), Token_Type::FN),
            (String::from("for"), Token_Type::FOR),
            (String::from("if"), Token_Type::IF),
            (String::from("impl"), Token_Type::IMPL),
            (String::from("let"), Token_Type::LET),
            (String::from("nil"), Token_Type::NIL),
            (String::from("print"), Token_Type::PRINT),
            (String::from("return"), Token_Type::RETURN),
             (String::from("struct"), Token_Type::STRUCT),
            (String::from("true"), Token_Type::TRUE),
            (String::from("while"), Token_Type::WHILE),
            (String::from("破"), Token_Type::BREAK),
            (String::from("跃"), Token_Type::Continue),
            (String::from("则"), Token_Type::ELSE),
            (String::from("假"), Token_Type::FALSE),
            (String::from("令"), Token_Type::FN),
            (String::from("令循环"), Token_Type::FOR),
            (String::from("若"), Token_Type::IF),
            (String::from("诏"), Token_Type::LET),
            (String::from("空"), Token_Type::NIL),
            (String::from("打印"), Token_Type::PRINT),
            (String::from("返回"), Token_Type::RETURN),
            (String::from("真"), Token_Type::TRUE),
            (String::from("序循环"), Token_Type::WHILE),
        ])
    };
}
impl Token {
    pub fn new(
        token_type: Token_Type,
        lexeme: String,
        literal: Object,
        line: usize,
    ) -> Token {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
    pub fn to_string(&self) -> String {
        self.token_type.to_string() + &self.lexeme +  &self.literal.to_string() + &self.line.to_string()
    }
}
