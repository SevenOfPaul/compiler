use crate::ast::token::token_type::Token_type;
use lazy_static::lazy_static;
use crate::ast::token::object::Object;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub(crate) token_type: Token_type,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Object>,
    pub(crate) line: usize,
}
lazy_static! {
    pub(crate) static ref Keywords: HashMap<String, Token_type> = {
        HashMap::from([
            (String::from("let"), Token_type::LET),
            (String::from("print"), Token_type::PRINT),
            (String::from("fn"), Token_type::FN),
            (String::from("nil"), Token_type::NIL),
            (String::from("true"), Token_type::TRUE),
            (String::from("false"), Token_type::FALSE),
        ])
    };
}
impl Token {
    pub fn new(
        token_type: Token_type,
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
