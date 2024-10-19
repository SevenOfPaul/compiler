use crate::Lexer;
use Lexer::lexer::Token_type;
struct Token{
    token_type:Token_type,
    lexeme:String,
    literal:String,
    line:i32,
}
impl Token{
    fn new(token_type:Token_type,lexeme:String,literal:String,line:i32)->Token{
        Self{token_type,lexeme,literal,line}
    }
    fn to_string(&self)->String{
        self.token_type.to_string()+&self.lexeme+&self.literal+&self.line.to_string()
    }
}