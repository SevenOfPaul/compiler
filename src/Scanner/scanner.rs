use crate::Token;
use Token::token_type::Token_type;
struct Scanner {
      source:Vec<char>,
      tokens: Vec<Token_type>,
    //start是被处理的第一个字符
      start:usize,
    //cur是当前字符 例如let start永远指向l cur可能为 l e t
      cur:usize,
      line: usize,
 }
impl Scanner {
    fn new(source:String) -> Scanner {
        //声明扫描器
         Self{source:source.chars().collect(),tokens:Vec::new(),start:0,cur:0,line:1}
    }
   /*
   递归整个源文件
   */
    fn scan_tokens(&mut self) ->Vec<Token_type>{
        //没到头
        while !self.is_at_end() {
            //递归下去
        self.start = self.cur;
        //识别每一个token
        self.scan_token();
        }
         //添加token
         self.tokens.clone()
    }
    fn scan_token(&mut self){
        //没到头
       let c=self.advance();
          match c {
             '('=>addToken(Token_type::LEFT_PAREN),
             ')'=> addToken(Token_type::RIGHT_PAREN),
             '{'=> addToken(Token_type::LEFT_BRACE),
             '}'=> addToken(Token_type::RIGHT_BRACE),
             ','=> addToken(Token_type::COMMA),
             '.'=> addToken(Token_type::DOT),
             '-'=>addToken(Token_type::MINUS),
             '+'=> addToken(Token_type::PLUS),
             ';'=>addToken(Token_type::SEMICOLON),
             '*'=> addToken(Token_type::STAR),
              _=>addToken(Token_type::EOF)
        }
    }
     fn add_token(&mut self ,token_type:Option<Token_type>) {
         if let Some(token) = token_type {

         }else{
             let text = &self.source[self.start..=self.cur];
             // tokens.add(new Token(type, text, literal, line));
         }
         // add_token(type, null);
    }
    fn is_at_end(&self)->bool {
        self.cur >= self.source.len()
    }
     fn advance(&mut self)->char {
         //返回当前指向的字符
         self.cur+=1;
         self.source[self.cur-1]
    }
}