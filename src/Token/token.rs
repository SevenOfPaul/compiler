use crate::Token::token_type::Token_type;
use crate::Token::object::Object;
#[derive(Clone, Debug)]
pub(crate) struct Token{
    token_type:Token_type,
    lexeme:String,
    literal:Option<Object>,
    line:usize,
}
impl Token{
    pub fn new(token_type:Token_type,lexeme:String,literal:Option<Object>,line:usize)->Token{
        Self{token_type,lexeme,literal,line}
    }
    pub fn to_string(&self)->String{
        let liter=if let Some(l)=&self.literal{&l.to_string()}else{""};
        self.token_type.to_string()+&self.lexeme+liter+&self.line.to_string()
    }
}