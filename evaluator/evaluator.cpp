#include <evaluator/evaluator.h>
#include <iostream>
using namespace lisp::evaluator;

shared_ptr<Object> Evaluator::eval(const shared_ptr<ast::Node> &node) {
    switch (node->get_type()) {
        case ast::Node::NODE_PROGRAM: {
            // 将node转换为Program
            return eval_program(std::dynamic_pointer_cast<ast::Program>(node)->statements);
        }
        case ast::Node::NODE_EXPRESSION_STATEMENT: {
            return eval(std::dynamic_pointer_cast<ast::ExpressionStatement>(node));
        }
        case ast::Node::NODE_INEGER: {
            return eval_integer(std::dynamic_pointer_cast<ast::Integer>(node));
        }
        case ast::Node::NODE_INFIX: {
            auto e = std::dynamic_pointer_cast<ast::Infix>(node);
            auto left = eval(e->left);
            auto right = eval(e->right);
            return eval_infix(e->operation, left, right);
        }
        case ast::Node::NODE_PREFIX: {
        }
        default: {
            std::cout << node->get_type() + "不存在";
        }
    }
}
shared_ptr<Object> Evaluator::eval_program(const list<shared_ptr<lisp::ast::Statement>> &stms) {
    std::shared_ptr<Object> result;
    for (auto &stat: stms) {
        result = eval(stat);
    }
    return result;
}
shared_ptr<Object> Evaluator::eval_integer(const shared_ptr<lisp::ast::Integer> &node) {
    return Object::new_integer(node->val);
}
shared_ptr<Object> Evaluator::eval_infix(const string &op, const std::shared_ptr<Object> &left,
                                         const std::shared_ptr<Object> right) {

}
