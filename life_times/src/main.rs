fn main() {
  let param1 = 1;
  let param2 = 2;

  let result = get_int_reference(&param1);
  println!("get_int_reference: {}", result);

  let result = get_int_reference2(&param1, &param2);
  println!("get_int_reference2: {}", result);

  let (result1, result2) = get_int_reference3(&param1, &param2);
  println!("get_int_reference3: {} {}", result1, result2);

  let (r1, r2) = get_int(3 , 4);
  println!("get_int: {} {}", r1, r2);
}

fn get_int_reference(param1: &i32) -> &i32 {
  println!("{}", param1);
  param1
}

// fn get_int_reference2(param1: &i32, param2: &i32) -> &i32 { // error
fn get_int_reference2<'a>(param1: &i32, param2: &'a i32) -> &'a i32 {
  println!("{} {}", param1, param2);
  param2
}

fn get_int_reference3<'a>(param1: &'a i32, param2: &'a i32) -> (&'a i32, &'a i32) {
  println!("{} {}", param1, param2);
  (param1, param2)
}

fn get_int(param1: i32, param2: i32) -> (i32, i32) {
  println!("{} {}", param1, param2);
  (param1, param2)
}
