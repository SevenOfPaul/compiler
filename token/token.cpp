#include <token/token.h>
using namespace lisp::token;
using std::string;
using std::unordered_map;
unordered_map<Token::Type, string> Token::names = {
        {TOKEN_ILLEGAL, "illegal"}, {TOKEN_EOF, "eof"},    {TOKEN_INTEGER, "interger"}, {TOKEN_PLUS, "+"},
        {TOKEN_MINUS, "-"},         {TOKEN_ASTERISK, "*"}, {TOKEN_SLASH, "/"},          {TOKEN_LPAREN, "("},
        {TOKEN_RPAREN, ")"},        {TOKEN_SEMICLON, ";"}};
Token::Token() : type(TOKEN_ILLEGAL) {}
Token::Token(Type type, const string &literal) : type(type), literal(literal) {}
Token::Type Token::get_type() const { return type; };
string Token::get_name() const {
    const auto it = names.find(type);
    if (it != names.end()) {
        return it->second;
    }
    return "";
};
string Token::get_literal() const { return literal; };
Token &Token::operator=(const Token &other) {
    if (this == &other) {
        return *this;
    }
    type = other.type;
    literal = other.literal;
}
