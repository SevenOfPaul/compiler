// impl Expr {
//     pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
//         match  self {
//              Expr::Literal {val}=>{
//                 visitor.visit_literal(val)
//             }
//             Expr::Grouping {expression}=>{
//                 visitor.visit_grouping(expression)
//             }
//             Expr::Binary {operator, l_expression, r_expression}=>{
//                 visitor.visit_binary(operator,l_expression,r_expression)
//             }
//             Expr::Unary {operator, r_expression}=>{
//                 visitor.visit_unary(operator,r_expression)
//             }
//         }
//     }
// }
use paste::paste;
use crate::ast::expression::expr::Expr;

#[macro_export]
macro_rules! impl_expr_accept {
    ($(
   ($label:ident,$func:ident,{$($params:ident,)*}),
    )*)=>{
        impl Expr{
      pub  fn accept<T>(&self, visitor: &mut dyn Visitor<T>)->T{
             match self {
               $(
                Expr::$label{$($params,)*} =>{
                    visitor.$func($($params,)*)
                }
               )*
             }
            }
        }
    }
}
