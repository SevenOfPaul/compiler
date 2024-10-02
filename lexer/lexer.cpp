#include <fstream>
#include <iostream>
#include <lexer/lexer.h>
#include <sstream>
using namespace lisp::lexer;
using namespace std;
Lexer::Lexer(){};
Lexer::Lexer(const string &file) {
    ch = 0;
    pos = 0;
    next_pos = 0;
    // 读取文件
    const ifstream ifs(file);
    if (!ifs.good()) {
        cout << "read file has some error" << file << endl;
        return;
    }
    // 把文件读取到oss中
    ostringstream oss;
    oss << ifs.rdbuf();
    // 把读取到的文件保存到input中
    input = oss.str();
};
void Lexer::skip_white_space() {
    while (ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r') {
        // 直接跳过
        read_char();
    }
}
void Lexer::read_char() {
    if (next_pos > input.length()) {
        // 到地方了 就结束
        ch = 0;
    } else {
        // 当前为下移，next 指针下移
        ch = input[next_pos];
    }
    pos=next_pos;
    next_pos++;
}
void Lexer::unread_char() {
    // 倒回去
    next_pos = pos;
    pos--;
};
bool Lexer::is_digit(char ch) { return (ch >= '0' && ch <= '9'); };
string Lexer::read_number() {
    int cur = pos;
    while (is_digit(ch)) {
        read_char();
    }
    return input.substr(cur, pos - cur);
};
Token Lexer::new_token(Token::Type type, const string &literal) {
    // 创建token 变量名为token
    Token token(type, literal);
    return token;
};
// 创建下一个token
Token Lexer::next_token() {
    read_char();
    skip_white_space();
    // 根据读到的数据匹配token
    // ch是char类型 需要类型转换
    switch (ch) {
        case '+': {
            string literal;
            literal += ch;
            return new_token(Token::TOKEN_PLUS, literal);
        }
        case '-': {
            string literal;
            literal += ch;
            return new_token(Token::TOKEN_MINUS, literal);
        }
        case '*': {
            string literal;
            literal += ch;
            return new_token(Token::TOKEN_ASTERISK, literal);
        }
        case '/': {
            string literal;
            literal += ch;
            return new_token(Token::TOKEN_SLASH, literal);
        }
        case '(': {
            string literal;
            literal += ch;
            return new_token(Token::TOKEN_LPAREN, literal);
        }
        case ')': {
            string literal;
            literal += ch;
            return new_token(Token::TOKEN_RPAREN, literal);
        }
        case 0: {
            return new_token(Token::TOKEN_EOF, "");
        }
        // 是不是数字
        default: {
            if (is_digit(ch)) {
                string integer = read_number();
                // 读取到不是数的时候会跳出 但是这时候位置超了1 太机智了
                unread_char();
                return new_token(Token::TOKEN_INTEGER,integer);
          }
            // std::cout<<ch;
              return new_token(Token::TOKEN_ILLEGAL,"");
        }
    }
};