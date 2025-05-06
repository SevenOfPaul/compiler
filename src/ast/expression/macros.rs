// impl Expr {
//     pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
//         match  self {
//              Expr::Literal {val}=>{
//                 visitor.visitor_literal(val)
//             }
//             Expr::Grouping {expression}=>{
//                 visitor.visitor_grouping(expression)
//             }
//             Expr::Binary {operator, l_expression, r_expression}=>{
//                 visitor.visitor_binary(operator,l_expression,r_expression)
//             }
//             Expr::Unary {operator, r_expression}=>{
//                 visitor.visitor_unary(operator,r_expression)
//             }
//         }
//     }
// }


#[macro_export]
macro_rules! impl_expr_accept {
    ($(
   ($label:ident,$func:ident,{$($params:ident,)*}),
    )*)=>{
        paste!{
        impl Expr{
      pub  fn accept<T>(&self, visitor: &mut dyn Visitor<T>)->T{
             match self {
               $(
                Expr::$label{$($params,)*} =>{
                    visitor.[<visitor_$func>]($($params,)*)
                }
               )*
             }
            }
        }
      }
    }
}
