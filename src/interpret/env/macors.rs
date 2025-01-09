#[macro_export]
macro_rules! match_type{
  ($name:expr,$typ:pat,$func1:block,$func2:block)=>{
    match $name {
      $typ => $func1,
      _ =>$func2
    }
  }
}