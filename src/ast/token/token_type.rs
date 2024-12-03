#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token_Type {
    //自己就是个token
    LEFT_PAREN,//(
    RIGHT_PAREN,//)
    LEFT_BRACE,//{
    RIGHT_BRACE,//}
    COMMA,//,
    DOT,//.
    MINUS, //-
    PLUS,//+
    SLASH,// /
    STAR, // *
    SEMICOLON,//;
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
    IF,//if
    WHILE,//while
    FOR,//for 循环
    /*
    注释不进入parse 这里只是方便理解
    COMMENT_START,// /*
    COMMENT_END, // */
    COMMENTED, // //
    */
    EOF //结尾符
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