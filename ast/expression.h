#pragma once
#include <ast/node.h>
#include <memory>
namespace lisp {
    namespace ast {
        class Expression : public Node {
        public:
            //构造函数默认 等价于   Expression() : Node() {}
            Expression(){}

            Expression(Type type) : Node(type) {}

            ~Expression() {}
        };
        class Integer : public Expression {
        public:
            Integer() : Expression(NODE_INEGER) {}
            ~Integer() {}
            int val;
        };
        class Infix : public Expression {
        public:
            Infix() : Expression(NODE_INFIX) {}
            ~Infix() {}
            string operation;
            std::shared_ptr<Expression> left;
            std::shared_ptr<Expression>  right;
        };
    } // namespace ast
} // namespace lisp
