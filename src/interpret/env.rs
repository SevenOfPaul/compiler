use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::ast::expression::expr::Expr;

lazy_static! {
    pub(crate) static ref Environment:HashMap<String,Expr>=HashMap::new();
}