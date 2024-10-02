
#pragma once
#include <ast/expression.h>
#include <ast/node.h>
#include <ast/program.h>
#include <object/object.h>
using namespace lisp::object;
namespace lisp {
    namespace evaluator {
        class Evaluator {
        public:
            Evaluator() {}
            ~Evaluator() {}
            shared_ptr<Object> eval(const shared_ptr<ast::Node> &node);
            shared_ptr<Object> eval_program(const list<shared_ptr<ast::Statement>> &Statement);
            shared_ptr<Object> eval_integer(const shared_ptr<ast::Integer> &node);
            // 求中缀表达式的值
            shared_ptr<Object> eval_infix(const string &op, const std::shared_ptr<Object> &left,
                                          const std::shared_ptr<Object> right);
        };
  }
}
