#pragma once
#include <ast/node.h>
#include <list>
#include <memory>
namespace lisp {
  namespace ast {
    class Program:public Statement {
    public:Program():Statement(NODE_EXPRESSION_STATEMENT){}
      ~Program(){}
      std::list<std::shared_ptr<Statement>> statements;
    };
  }
}
