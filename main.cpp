#include <evaluator/evaluator.h>
#include <iostream>
#include <lexer/lexer.h>
#include <memory>
#include <parser/parser.h>

int main() {
    std::shared_ptr<lisp::lexer::Lexer> lexer(new lisp::lexer::Lexer("./../main.lisp"));
    std::shared_ptr<lisp::parser::Parser> parser(new lisp::parser::Parser(lexer));
   auto program=parser->parse_progam();
   // std::shared_ptr<lisp::evaluator::Evaluator> evaluator(new lisp::evaluator::Evaluator);
   // auto evaluated =evaluator->eval(program);
   // if(evaluated) {
   //     std::cout<<evaluated->str()<<endl;
   // }
    return 0;
}
