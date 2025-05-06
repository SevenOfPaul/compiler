#[derive(Debug)]
pub enum FN_TYPE {
FN ,
None
}
impl PartialEq for FN_TYPE {
    fn eq(&self, other: &Self) -> bool {
       match self {
           FN_TYPE::FN=>{
               if let FN_TYPE::FN=other {
                   return true
               }
           },
           _=>{
               if let FN_TYPE::None=other {
                  return true
               }
           }
       }
        false
    }
}