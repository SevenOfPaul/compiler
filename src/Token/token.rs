use crate::Token::token_type::Token_type;
#[derive(Clone, Debug)]
pub(crate) struct Token{
    token_type:Token_type,
    lexeme:String,
    literal:Option<String>,
    line:usize,
}
impl Token{
    pub fn new(token_type:Token_type,lexeme:String,literal:Option<String>,line:usize)->Token{
        Self{token_type,lexeme,literal,line}
    }
    pub fn to_string(&self)->String{
        let liter=if let Some(l)=&self.literal{l}else{""};
        self.token_type.to_string()+&self.lexeme+liter+&self.line.to_string()
    }
}