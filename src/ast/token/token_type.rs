#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token_Type {
    //自己就是个token
    AND, //&&
    BANG, // !
    BANG_EQUAL,//！=
    BREAK,//break
    COLON,//:
    COMMA,//,
    Continue,
    DOT,//.
    ELSE,//else
    EOF, //结尾符
    EQUAL, //=
    EQUAL_EQUAL,//==
    FALSE,
    FOR,//for 循环
    FN,//函数
    GREATER, //>
    GREATER_EQUAL, //>=
    IDENTIFIER, //字面量
    IF,//if
    LEFT_BRACE,//{
    LEFT_PAREN,//(
    LESS,//<
    LESS_EQUAL, //<=
    LET, //声明
    MINUS, //-
    NIL, //null
    NUMBER, //数字
    OR, //||
    PLUS,//+
    PRINT, //打印
    QUESTION,//?
    RIGHT_BRACE,//}
    RIGHT_PAREN,//)
    RETURN,//return
    STRING, //字符串
    SLASH,// /
    STAR,// *
    SEMICOLON, //;
    TRUE,//True
    WHILE,//while
}
impl Token_Type {
    pub(crate) fn to_string(&self) ->String{
        String::from(match self {
            Token_Type::LEFT_PAREN => "LEFT_PAREN",
            Token_Type::RIGHT_PAREN => "RIGHT_PAREN",
            Token_Type::LEFT_BRACE => "LEFT_BRACE",
            Token_Type::RIGHT_BRACE => "RIGHT_BRACE",
            Token_Type::COMMA => "COMMA",
            Token_Type::MINUS => "MINUS",
            Token_Type::PLUS => "PLUS",
            Token_Type::SEMICOLON => "SEMICOLON",
            Token_Type::SLASH => "SLASH",
            Token_Type::STAR => "STAR",
            Token_Type::IDENTIFIER => "IDENTIFIER",
            Token_Type::BANG => "BANG",
            Token_Type::EQUAL => "EQUAL",
            Token_Type::BANG_EQUAL => "BANG_EQUAL",
            Token_Type::EQUAL_EQUAL => "EQUAL_EQUAL",
            Token_Type::GREATER => "GREATER",
            Token_Type::GREATER_EQUAL => "GREATER_EQUAL",
            Token_Type::LESS => "LESS",
            Token_Type::LESS_EQUAL => "LESS_EQUAL",
            Token_Type::AND => "AND",
            Token_Type::OR => "OR",
            Token_Type::LET => "LET",
            Token_Type::PRINT => "PRINT",
            Token_Type::FN => "FN",
            Token_Type::STRING => "STRING",
            Token_Type::NUMBER => "NUMBER",
            Token_Type::NIL => "NIL",
            _ => "EOF"
        })
    }
}