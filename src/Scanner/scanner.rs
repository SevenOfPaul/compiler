use crate::Error;
use crate::Token;
use Token::token;
use Token::token_type::Token_type;
use Token::object::Object;
pub(crate) struct Scanner {
    source: Vec<char>,
    tokens: Vec<token::Token>,
    //start是被处理的第一个字符
    start: usize,
    //cur是当前字符 例如let start永远指向l cur可能为 l e t
    cur: usize,
    line: usize,
}
impl Scanner {
    pub(crate) fn new(source: String) -> Scanner {
        //声明扫描器
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            cur: 0,
            line: 1,
        }
    }
    /*
    递归整个源文件
    */
    pub(crate) fn scan_tokens(&mut self) -> Vec<token::Token> {
        //没到头
        while !self.is_at_end() {
            //递归下去
            self.start = self.cur;
            //识别每一个token
            self.scan_token();
        }
        self.tokens.push(token::Token::new(
            Token_type::EOF,
            String::from(""),
            None,
            self.line,
        ));
        //添加token
        self.tokens.clone()
    }
    fn scan_token(&mut self) {
        //没到头
        let c = self.advance();
        match c {
            '(' => self.add_token(Token_type::LEFT_PAREN, None),
            ')' => self.add_token(Token_type::RIGHT_PAREN, None),
            '{' => self.add_token(Token_type::LEFT_BRACE, None),
            '}' => self.add_token(Token_type::RIGHT_BRACE, None),
            ',' => self.add_token(Token_type::COMMA, None),
            '.' => self.add_token(Token_type::DOT, None),
            '-' => self.add_token(Token_type::MINUS, None),
            '+' => self.add_token(Token_type::PLUS, None),
            ';' => self.add_token(Token_type::SEMICOLON, None),
            '*' => self.add_token(Token_type::STAR, None),
            //以上为单字符 还有双字符
            '!' => {
                let tok = if self.is_match('=') {
                    Token_type::BANG_EQUAL
                } else {
                    Token_type::BANG
                };
                self.add_token(tok, None);
            }
            '=' => {
                let tok = if self.is_match('=') {
                    Token_type::EQUAL_EQUAL
                } else {
                    Token_type::EQUAL
                };
                self.add_token(tok, None);
            }
            '<' => {
                let tok = if self.is_match('=') {
                    Token_type::LESS_EQUAL
                } else {
                    Token_type::LESS
                };
                self.add_token(tok, None);
            }
            '>' => {
                let tok = if self.is_match('=') {
                    Token_type::GREATER_EQUAL
                } else {
                    Token_type::GREATER
                };
                self.add_token(tok, None);
            }
            '/' => {
                //说明是注释
                if self.is_match('/') {
                    //注释就跳过
                    while !self.is_at_end() && self.peek() != '\n' {
                        //是注释就跳过 等于
                        // while !self.is_at_end()&&self.is_match{
                        // self.advance();
                        //}
                        self.advance();
                    }
                } else {
                    self.add_token(Token_type::SLASH, None);
                };
            }
            //这几个无意义
            '\t' => {}
            ' ' => {}
            '\r' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.get_string();
                //字符串
            }
            //全都没有那就报错把
            _ => {
                //看看是不是个数字
                if Self::is_digit(c){
                 self.get_number();
                }else {
                    Error::log(
                        self.line,
                        &*("Unexpected character".to_owned() + &*c.to_string()),
                    );
                }
            }
        }
    }
    fn add_token(&mut self, token_type: Token_type, literal: Option<Object>) {
        let text = &self.source[self.start..self.cur].iter().collect::<String>();
        self.tokens.push(token::Token::new(
            token_type,
            text.clone(),
            literal,
            self.line,
        ));
    }
    fn advance(&mut self) -> char {
        //返回当前指向的字符
        self.cur += 1;
        self.source[self.cur - 1]
    }
    //看看下个是啥，不会增加cur
    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.cur]
        }
    }
    //在多看一个
    fn peek_next(&mut self) -> char {
        //到头了
        if self.cur + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.cur + 1]
        }
    }
    //这里得大改
    fn get_number(&mut self) {
       while Self::is_digit(self.peek()){self.advance();}
       if self.peek()=='.'&& Self::is_digit(self.peek_next()){
           self.advance();
       }
        while Self::is_digit(self.peek()){self.advance();}
        let val: String = self.source[self.start..self.cur].iter().collect();
        self.add_token(Token_type::NUMBER, Some(Object::num(val.parse().unwrap())));
    }
    fn get_string(&mut self) {
        while self.peek()!='"'&&!self.is_at_end() {
            //跳过换行
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }
        self.advance();
        //没找到 后面的"
        if self.is_at_end(){
            Error::log(self.line, "Unterminated string.");
        }
        let val: String = self.source[self.start+1..self.cur-1].iter().collect();
        self.add_token(Token_type::STRING, Some(Object::str(val)));
    }
    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.cur] != expected {
            false
        } else {
            self.cur += 1;
            true
        }
    }
    fn is_digit(c:char)->bool{
        c >= '0' && c <= '9'
    }
    fn is_at_end(&self) -> bool {
        self.cur >= self.source.len()
    }
}
