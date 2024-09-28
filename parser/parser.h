#pragma once
#include <ast/expression.h>
#include <lexer/lexer.h>
#include <list>
#include <memory>
namespace lisp {
    namespace parser {
        class Parser {
        public:
            enum Precedence {
                LOWEST = 0,
                SUM, //+ -
                PROODCT, //* /
            };
            Parser();
            Parser(const std::shared_ptr<lexer::Lexer>& lexer);
            ~Parser();
            //定义前缀表达式构建函数 重点 typedef定义了一个函数指针 保证这个指针可以被引用
             typedef std::shared_ptr<ast::Expression>(Parser::*prefix_parse_fn)(void);
            //定义中缀表达式构建函数 重点
            typedef std::shared_ptr<ast::Expression>(Parser::*infix_parse_fn)(const std::shared_ptr<ast::Expression> & pre);
            //获取下一个符号
            void next_token();
            bool cur_token_is(Token::Type type);
            bool next_token_is(Token::Type type);
            //下一个token是不是所期望的 是在进入下一个 不是就报错
            bool expect_next_token(Token::Type type);
            void next_error(Token::Type type);
            int get_cur_token_precedence();
            int get_next_token_precedence();
            //解析整数
            std::shared_ptr<ast::Expression> parse_integer();
            std::shared_ptr<ast::Expression> parse_group();
            //中缀表达式
            std::shared_ptr<ast::Expression> parse_expression(int pracedence);

            //中缀表达式 传入一个表达式
        std::shared_ptr<ast::Expression> parse_infix(const std::shared_ptr<ast::Expression> &left);
            std::shared_ptr<ast::Program> parse_progam();
            std::shared_ptr<ast::Statement> parse_statment();
        private:
            std::shared_ptr<lexer::Lexer> lexer;
            //当前的符号 和下一个符号 以保证优先级
            Token cur;
            Token next;
            std::list<string> errors;
            static std::unordered_map<Token::Type,int> precedences;
            static std::unordered_map<Token::Type,prefix_parse_fn> prefix_parse_fns;
            static std::unordered_map<Token::Type,infix_parse_fn> infix_parse_fns;
        };
    } // namespace parser
} // namespace lisp
