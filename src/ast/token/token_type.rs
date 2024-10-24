#[derive(Clone, Debug)]
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
    IDENTIFIER, //字面量
    STRING, //字符串
    NUMBER, //数字
    AND, //&&
    OR, //||
    //关键字
    LET, //声明
    PRINT, //打印
    FN,//函数
    NIL, //null
    TRUE,
    FALSE,
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
            Token_type::BANG => "BANG",
            Token_type::EQUAL => "EQUAL",
            Token_type::BANG_EQUAL => "BANG_EQUAL",
            Token_type::EQUAL_EQUAL => "EQUAL_EQUAL",
            Token_type::GREATER => "GREATER",
            Token_type::GREATER_EQUAL => "GREATER_EQUAL",
            Token_type::LESS => "LESS",
            Token_type::LESS_EQUAL => "LESS_EQUAL",
            Token_type::AND => "AND",
            Token_type::OR => "OR",
            Token_type::LET => "LET",
            Token_type::PRINT => "PRINT",
            Token_type::FN => "FN",
            Token_type::STRING => "STRING",
            Token_type::NUMBER => "NUMBER",
            Token_type::NIL => "NIL",
            _ => "EOF"
        })
    }
}