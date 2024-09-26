cmake_minimum_required(VERSION 3.28)
project(compiler)

set(CMAKE_CXX_STANDARD 11)
include_directories(./)
file(GLOB_RECURSE "token/*.cpp" "lexer/*.cpp" "ast/*.cpp" "parser/*.cpp")
add_executable(main ${SOURCES} main.cpp token/token.cpp lexer/lexer.cpp ast/node.cpp ast/expression.cpp
        ast/program.cpp
        parser/parser.cpp)
