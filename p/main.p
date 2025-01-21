/*fn Ok(n) {
  if (n < 2){ return n;}
  return Ok(n) + Ok(n - 2); 
}

let before = now();
print Ok(40);
let after = now();
print after - before;*/

fn Ok(n){
  if(n<2){
    return n;
  }
    return Ok(n-1)+Ok(n-2);
}
print Ok(50);