#pragma once
#include <string>
#include <token/token.h>
using namespace lisp::token;

namespace lisp {
    namespace lexer {
        class Lexer {
        public:
            Lexer();

            Lexer(const string &file);

            ~Lexer() = default;

            Token next_token();

            Token new_token(Token::Type type, const string &literal);

        private:
            void skip_white_space();

            // 读取字符
            void read_char();

            void unread_char();

            bool is_digit(char ch);

            string read_number();
            string input;
            char ch;
            int pos;
            int next_pos;
        };
    }
}
