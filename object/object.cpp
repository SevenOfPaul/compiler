#include <ast/node.h>
#include <cstdarg>
#include <object/object.h>
#include <string.h>
using namespace lisp::object;
using namespace std;
unordered_map<Object::Type,string> Object::names={
{OBJECT_ERROR,"error"},
{OBJECT_INTEGER,"integer"}}
;
string Object::get_name(Type type) const {
    auto it=names.find(type);
    if(it!=names.end()) {
        return it->second;
    }
    return "";
}
 std::shared_ptr<Object> Object::new_error(const char *format,...) {
     char buf[1024]={{0}};
    va_list arg_ptr;
    va_start(arg_ptr,format);
    vsnprintf(buf,sizeof(buf),format,arg_ptr);
    va_end(arg_ptr);
   return std::shared_ptr<Error> (new Error(buf));
};
 std::shared_ptr<Object>  Object::new_integer(int val) {
return shared_ptr<Integer>(new Integer(val));
};
