#include <ast/node.h>
#include <ast/expression.h>
        //把表达式包装成为一条语句 这样就可以存储到根节点了
//表达式有值 语句没有
namespace lisp {
  namespace ast {
    class ExpressionStatement : public Node {
    public:
      ExpressionStatement(){}

      ExpressionStatement(Type type) : Node(NODE_EXPRESSION_STATEMENT) {}

      ~ExpressionStatement() {}
      std::shared_ptr<Expression> expression;
    };
  }
}