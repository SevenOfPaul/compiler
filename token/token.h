#pragma once
#include <string>
#include <unordered_map>
using std::string;
using std::unordered_map;
namespace lisp {
    namespace token {
        class Token {
        public:
            enum Type {
                TOKEN_ILLEGAL = 0, // 无效符
                TOKEN_EOF, // 停止符
                TOKEN_INTEGER, // 整数
                TOKEN_PLUS, //+
                TOKEN_MINUS, //-
                TOKEN_ASTERISK, //*
                TOKEN_SLASH, // /
                TOKEN_LPAREN, // (
                TOKEN_RPAREN, // )
                TOKEN_SEMICLON // ;
            };
            Token();
            Token(Type type, const string &literal);
            ~Token() = default;
            Type get_type() const;
            string get_name() const;
            string get_literal() const;
            Token &operator=(const Token &other);

        private:
            Type type;
            string literal;
            static unordered_map<Type, string> names;
};
}
}
