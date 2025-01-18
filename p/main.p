return "hello";
let a = "global";
{
  fn show(){
    print a;
  }

  show();
  let a = "block";
  show();
}