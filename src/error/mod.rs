mod report;

pub use report::*;
use crate::interpret::{Return, Run_Err};
use crate::parse::{Break, Continue, Parse_Err};

#[derive(Debug)]
pub (crate) enum X_Err{
    parse(Parse_Err),
    run(Run_Err),
    ret(Return),
    brk(Break),
    cte(Continue)
}



