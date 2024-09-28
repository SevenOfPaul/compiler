
#pragma once;
#include <unordered_map>
#include <string>
using namespace std;
namespace lisp {
    namespace object {
        class Object {
        public:
            enum Type { OBJECT_ERROR = 0, OBJECT_INTEGER };
            Object(){}
            Object(Type type):type(type){}
            virtual ~Object(){}
            Type get_type() const {return type;}
            string get_name() const;
            virtual string str() const=0;
        private :
            Type type;
            static unordered_map<Type,string> names;
        };
    } // namespace object
} // namespace lisp
