use crate::ast::token::token_type::Token_Type;
use lazy_static::lazy_static;
use crate::ast::token::object::Object;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub(crate) token_type: Token_Type,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Object>,
    pub(crate) line: usize,
}
lazy_static! {
    pub(crate) static ref Keywords: HashMap<String, Token_Type> = {
        HashMap::from([
            (String::from("let"), Token_Type::LET),
            (String::from("print"), Token_Type::PRINT),
            (String::from("fn"), Token_Type::FN),
            (String::from("nil"), Token_Type::NIL),
            (String::from("true"), Token_Type::TRUE),
            (String::from("false"), Token_Type::FALSE),
            (String::from("if"), Token_Type::IF),
             (String::from("else"), Token_Type::ELSE),
             (String::from("while"), Token_Type::WHILE),
             (String::from("for"),Token_Type::FOR)
        ])
    };
}
impl Token {
    pub fn new(
        token_type: Token_Type,
        lexeme: String,
        literal: Option<Object>,
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
        let liter = if let Some(l) = &self.literal {
            &l.to_string()
        } else {
            ""
        };
        self.token_type.to_string() + &self.lexeme + liter + &self.line.to_string()
    }
}
