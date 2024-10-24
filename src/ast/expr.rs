use crate::token::token_type::Token_type;
pub(crate) enum Expr {
    AssignExpr(Assign),
    BinaryExpr(Binary),
    GroupingExpr(Grouping),
}
struct Assign {
    name: Token_type,
    val: Box<Expr>,
}
struct Binary {
    name: Token_type,
    left: Box<Expr>,
    right: Box<Expr>,
}
struct Grouping {
    expression: Box<Expr>,
}
impl Assign {
    fn accept(self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_assign(&self);
    }
}
impl Binary {
    fn accept<T>(self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_binary(&self);
    }
}
impl Grouping {
    fn accept<T>(self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_grouping(&self);
    }
}
pub trait Visitor<T> {
    fn visit_assign(&self, expr: &Assign) -> T;
    fn visit_binary(&self, expr: &Binary) -> T;
    fn visit_grouping(&self, expr: &Grouping) -> T;
}
