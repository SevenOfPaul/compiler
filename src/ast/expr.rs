use crate::ast::token::token_type::Token_type;
use crate::ast::token::object::Object;
pub(crate) enum Expr {
    BinaryExpr(Binary),
    GroupingExpr(Grouping),
}
struct Binary {
    operator: Token_type,
    left_expression: Box<Expr>,
    right_expression: Box<Expr>,
}
struct Grouping {
    expression: Box<Expr>,
}
struct Literal {
    val:Option<Object>,
}
struct Unary{
    operator: Token_type,
    expression: Box<Expr>
}
impl Binary {
    fn accept<T>(self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_binary(&self)
    }
}
impl Grouping {
    fn accept<T>(self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_grouping(&self)
    }
}
pub trait Visitor<T> {
    fn visit_binary(&self, expr: &Binary) -> T;
    fn visit_grouping(&self, expr: &Grouping) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
    fn visit_unary(&self, expr: &Unary) -> T;
}
