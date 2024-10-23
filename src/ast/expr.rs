use crate::token::token_type::Token_type;
// abstract class Expr {
// interface Visitor<R> {
// R visitAssignExpr(Assign expr);
// R visitBinaryExpr(Binary expr);
// R visitCallExpr(Call expr);
// R visitGetExpr(Get expr);
// R visitGroupingExpr(Grouping expr);
// R visitLiteralExpr(Literal expr);
// R visitLogicalExpr(Logical expr);
// R visitSetExpr(Set expr);
// R visitSuperExpr(Super expr);
// R visitThisExpr(This expr);
// R visitUnaryExpr(Unary expr);
// R visitVariableExpr(Variable expr);
// }
pub (crate) enum Expr{
    AssignExpr(Assign),
    BinaryExpr(Binary),
}
struct Assign{
    name:Token_type,val:Box<Expr>
}
struct Binary{
    name:Token_type,left:Box<Expr>,right:Box<Expr>
}
impl Assign{
    fn new(name:Token_type, val:Box<Expr>) -> Assign{
        Self{name,val}
    }
}
impl Binary{
    fn new(name:Token_type, left:Box<Expr>,right:Box<Expr>) -> Self{
       Self {name,left,right}
    }
}