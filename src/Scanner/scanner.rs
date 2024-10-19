use crate::Error;
use crate::Token;
use Token::token;
use Token::token_type::Token_type;
struct Scanner {
    source: Vec<char>,
    tokens: Vec<token::Token>,
    //start是被处理的第一个字符
    start: usize,
    //cur是当前字符 例如let start永远指向l cur可能为 l e t
    cur: usize,
    line: usize,
}
impl Scanner {
    fn new(source: String) -> Scanner {
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
    fn scan_tokens(&mut self) -> Vec<token::Token> {
        //没到头
        while !self.is_at_end() {
            //递归下去
            self.start = self.cur;
            //识别每一个token
            self.scan_token();
        }
        self.tokens.push(token::Token::new(
            Token::token_type::Token_type::EOF,
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
            '!' => {
                let tok = if self.is_match('=') {
                    Token_type::GREATER_EQUAL
                } else {
                    Token_type::GREATER
                };
                self.add_token(tok, None);
            }
            //全都没有那就报错把
            _ => {
                Error::log(self.line, "Unexpected character.");
            }
        }
    }
    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.cur] != expected {
            false
        } else {
            self.cur += 1;
            true
        }
    }
    fn add_token(&mut self, token_type: Token_type, literal: Option<String>) {
        let text = &self.source[self.start..=self.cur]
            .iter()
            .collect::<String>();
        self.tokens.push(token::Token::new(
            token_type,
            text.clone(),
            literal,
            self.line,
        ));
    }
    fn is_at_end(&self) -> bool {
        self.cur >= self.source.len()
    }
    fn advance(&mut self) -> char {
        //返回当前指向的字符
        self.cur += 1;
        self.source[self.cur - 1]
    }
}
