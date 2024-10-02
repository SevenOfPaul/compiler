#pragma once
#include <memory>
#include <string>
#include <unordered_map>
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
            string get_name(Type type) const;
            virtual string str() const=0;
            //报错信息
            static std::shared_ptr<Object> new_error(const char *format,...);
            static std::shared_ptr<Object>  new_integer(int val);
        private :
            Type type;
            static unordered_map<Type,string> names;
        };
        class Integer :public Object {
        public: Integer():Object(OBJECT_INTEGER),val(0){}
            Integer(int val):Object(OBJECT_INTEGER),val(val){}
            virtual ~Integer(){}
            string str()const {
             return std::to_string(val);
        } ;
        public:
            int val;
        };
        class Error :public Object {
        public: Error():Object(OBJECT_ERROR){}
            Error(string message):Object(OBJECT_ERROR),message(message){}
            virtual ~Error(){}
            string str()const {
            return "error:"+message;
        } ;
        public:
            string message;
        };
    } // namespace object
} // namespace lisp
