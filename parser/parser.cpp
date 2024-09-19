#include <lexer/lexer.h>
#include <parser/parser.h>
#include <sstream>
using namespace lisp::parser;
using namespace lisp::lexer;
using namespace lisp::ast;
Parser::Parser() {}
Parser::Parser(const std::shared_ptr<Lexer> &lexer) {
    // 调用两次 第一次定义 tokencur  第二次调用获取toen next
    next_token();
    next_token();
}
Parser::~Parser() {}
std::unordered_map<Token::Type, int> Parser::precedences = {{Token::TOKEN_PLUS, SUM},
                                                            {Token::TOKEN_MINUS, SUM},
                                                            {Token::TOKEN_ASTERISK, PROODCT},
                                                            {Token::TOKEN_SLASH, PROODCT}};
void Parser::next_token() {
    cur = next;
    // 调用词法分析的nextToken 生成下一个token
    next = lexer->next_token();
}
bool Parser::cur_token_is(Token::Type type) { return cur.get_type() == type; }
bool Parser::next_token_is(Token::Type type) { return next.get_type() == type; }
void Parser::next_error(Token::Type type) {
    std::ostringstream oss;
    oss << "expected next token to be" << type << ", but got" << next.get_type() << "instead";
    errors.push_back(oss.str());
}
bool Parser::expect_next_token(Token::Type type) {
    if (next_token_is(type)) {
        next_token();
        return true;
    }
    // 抛出错误
    next_error(type);
    return false;
}
int Parser::get_cur_token_precedence() {
    auto it = precedences.find(cur.get_type());
    if (it != precedences.end()) {
        return it->second;
    }
    return LOWEST;
};
int Parser::get_next_token_precedence() {
    auto it = precedences.find(next.get_type());
    if (it != precedences.end()) {
        return it->second;
    }
    return LOWEST;
}
void Parser::no_prefix_parse_fn_error(Token::Type type) {
std::ostringstream oss;
    oss<<"no_prefix_parse_fn_error"<<type;
    errors.push_back(oss.str());
}
//中缀表达式
std::shared_ptr<Expression> Parser::parse_infix(const std::shared_ptr<ast::Expression> &left) {
 std::shared_ptr<Expression> e(new Infix());
    e->token=cur;
    e->operation=cur.get_literal();
    e->left

}

