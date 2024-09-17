#include <ast/node.h>
#include <unordered_map>
using namespace lisp::ast;
std::unordered_map<Node::Type, string> Node::names = {
        {NODE_INEGER, "Integer"},
        {NODE_INFIX, "Infix"},
        {NODE_EXPRESSION_STATEMENT, "ExpressionStatement"},
        {NODE_PROGRAM, "Program"},
        {NODE_PREFIX, "Prefix"},
};

string Node::get_name() const {
    auto it = names.find(type);
    if (it != names.end()) {
        return it->second;
    }
    return "";
}
