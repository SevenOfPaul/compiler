#[derive(Clone)]
pub enum Token_type{
    //自己就是个token
    LEFT_PAREN,//(
    RIGHT_PAREN,//)
    LEFT_BRACE,//{
    RIGHT_BRACE,//}
    COMMA,//,
    DOT,//.
    MINUS, //-
    PLUS,//+
    SEMICOLON,//*
    SLASH,// /
    STAR, // *
    BANG, // !
    EQUAL, //=
    //两个字符构成的token
    BANG_EQUAL,//！=
    EQUAL_EQUAL,//==
    GREATER, //>
    GREATER_EQUAL, //>=
    LESS,//<
    LESS_EQUAL, //<=
    //字面量
    IDENTIFIER, //标识符
    STRING, //字符串
    NUMBER, //数字
    EOF //结尾符
}
impl Token_type{
    pub(crate) fn to_string(&self) ->String{
        String::from(match self {
            Token_type::LEFT_PAREN => "LEFT_PAREN",
            Token_type::RIGHT_PAREN => "RIGHT_PAREN",
            Token_type::LEFT_BRACE => "LEFT_BRACE",
            Token_type::RIGHT_BRACE => "RIGHT_BRACE",
            Token_type::COMMA => "COMMA",
            Token_type::MINUS => "MINUS",
            Token_type::PLUS => "PLUS",
            Token_type::SEMICOLON => "SEMICOLON",
            Token_type::SLASH => "SLASH",
            Token_type::STAR => "STAR",
            Token_type::IDENTIFIER => "IDENTIFIER",
            Token_type::STRING => "STRING",
            Token_type::NUMBER => "NUMBER",
            Token_type::EOF => "EOF",
            _ => "EOF"
        })
    }
}