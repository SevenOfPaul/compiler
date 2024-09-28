#pragma once
#include <ast/node.h>
#include <ast/expression_statement.h>
#include <list>
#include <memory>
namespace lisp {
  namespace ast {
    class Program:public Statement {
    public:Program():Statement(Node::NODE_EXPRESSION_STATEMENT){}
      ~Program(){}
      std::list<std::shared_ptr<Statement>> statements;
    };
  }
}
