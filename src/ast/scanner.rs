use crate::ast::token::object::Object;
use crate::ast::token::token::{self, Keywords};
use crate::ast::token::token_type::Token_Type;
use chinese_detection::{ClassificationResult, classify};
use crate::error;

#[derive(Debug)]
pub(crate) struct Scanner {
    //cur是当前字符 例如let start永远指向l cur可能为 l e t
    cur: usize,
    line: usize,
    source: Vec<char>,
    //start是被处理的第一个字符
    start: usize,
    tokens: Vec<token::Token>,
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
    fn add_token(&mut self, token_type: Token_Type, literal: Object) {
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
    fn get_identifier(&mut self) {
        while  !self.is_at_end()&&Self::is_alaph_or_digit_or_chinese(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.cur].iter().collect::<String>();
        if let Some(token_type) = Keywords.get(&text) {
            self.add_token(token_type.clone(), Object::Str(text));
        } else {
            self.add_token(Token_Type::IDENTIFIER, Object::Str(text));
        }
    }
    fn get_number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
        }
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        let val: String = self.source[self.start..self.cur].iter().collect();
        self.add_token(Token_Type::NUMBER, Object::Num(val.parse().unwrap()));
    }
    fn get_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            //跳过换行
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }
        //没找到 后面的"
        if self.is_at_end() {
            error::log(self.line, &self.peek().to_string(), "缺失部分");
            return;
        }
        self.advance();
        let val: String = self.source[self.start + 1..self.cur - 1].iter().collect();
        self.add_token(Token_Type::STRING, Object::Str(val));
    }
    fn is_alaph(c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
                (c >= 'A' && c <= 'Z') ||
                //为什么有个_
                c == '_'
    }
    fn is_alaph_or_digit_or_chinese(c: char) -> bool {
        Self::is_alaph(c) || Self::is_digit(c)||Self::is_chinese(c)
    }
    fn is_chinese(c:char)->bool{
       classify(&c.to_string())== ClassificationResult::ZH
    }
    fn is_at_end(&self) -> bool {
        self.cur >= self.source.len()
    }
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.cur] != expected {
            false
        } else {
            self.cur += 1;
            true
        }
    }
    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.cur]
        }
    }
    fn peek_next(&mut self) -> char {
        //到头了
        if self.cur + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.cur + 1]
        }
    }
    pub(crate) fn scan_tokens(&mut self) -> Vec<token::Token> {
        //没到头
        while !self.is_at_end() {
            //递归下去
            self.start = self.cur;
            //识别每一个token
            self.scan_token();
        }
        self.tokens.push(token::Token::new(
            Token_Type::EOF,
            String::from(""),
            Object::Nil,
            self.line,
        ));
        //添加token
        self.tokens.clone()
    }
    fn scan_token(&mut self) {
        //没到头
        let c = self.advance();
        match c {
            '(' => self.add_token(Token_Type::LEFT_PAREN, Object::Nil),
            ')' => self.add_token(Token_Type::RIGHT_PAREN, Object::Nil),
            '{' => self.add_token(Token_Type::LEFT_BRACE, Object::Nil),
            '}' => self.add_token(Token_Type::RIGHT_BRACE, Object::Nil),
            ',' => self.add_token(Token_Type::COMMA, Object::Nil),
            '.' => self.add_token(Token_Type::DOT, Object::Nil),
            '-' => self.add_token(Token_Type::MINUS, Object::Nil),
            '+' => self.add_token(Token_Type::PLUS, Object::Nil),
            ';' => self.add_token(Token_Type::SEMICOLON, Object::Nil),
            '*' => self.add_token(Token_Type::STAR, Object::Nil),
            '?' => self.add_token(Token_Type::QUESTION, Object::Nil),
            ':' => self.add_token(Token_Type::COLON, Object::Nil),
            '&' => {
                if self.is_match('&') {
                    self.add_token(Token_Type::AND, Object::Nil)
                } else {
                    error::log(self.line, &self.peek().to_string(), "暂不支持位与运算");
                }
            }
            '|' => {
                if self.is_match('|') {
                    self.add_token(Token_Type::OR, Object::Nil)
                } else {
                    error::log(self.line, &self.peek().to_string(), "暂不支持位或运算");
                }
            }
            //以上为单字符 还有双字符
            '!' => {
                let tok = if self.is_match('=') {
                    Token_Type::BANG_EQUAL
                } else {
                    Token_Type::BANG
                };
                self.add_token(tok, Object::Nil);
            }
            '=' => {
                let tok = if self.is_match('=') {
                    Token_Type::EQUAL_EQUAL
                } else {
                    Token_Type::EQUAL
                };
                self.add_token(tok, Object::Nil);
            }
            '<' => {
                let tok = if self.is_match('=') {
                    Token_Type::LESS_EQUAL
                } else {
                    Token_Type::LESS
                };
                self.add_token(tok, Object::Nil);
            }
            '>' => {
                let tok = if self.is_match('=') {
                    Token_Type::GREATER_EQUAL
                } else {
                    Token_Type::GREATER
                };
                self.add_token(tok, Object::Nil);
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
                } else if self.is_match('*') {
                    //多行注释
                    while !self.is_at_end() {
                        if self.is_match('*') && self.peek() == '/' {
                            break;
                        }
                        self.advance();
                    }
                    //因为下一个是 / 所以还需要再走一步
                    self.advance();
                    if self.is_at_end() {
                        error::log(self.line, &self.peek().to_string(), "需要*/");
                    }
                } else {
                    self.add_token(Token_Type::SLASH, Object::Nil);
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
                if Self::is_digit(c) {
                    self.get_number();
                } else if Self::is_alaph_or_digit_or_chinese(c) {
                    //看看是不是关键字
                    self.get_identifier();
                } else {
                    error::log(
                        self.line,
                        &self.peek().to_string(),
                        &*("无效字符".to_owned() + &*c.to_string()),
                    );
                }
            }
        }
    }
}
