fn printf(num){
    num;
}
struct Obj{printf};
let obj=Obj{printf:printf};
let start=now();
for(let a=0;a<100000;a=a+1){
    obj.a=a;
    obj.printf(obj.a);
}
let end=now();
print String(end-start) +"ms";