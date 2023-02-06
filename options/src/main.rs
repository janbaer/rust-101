fn main() {
  let int_array = [1, 2, 3];
  println!("ArrayItem {}", int_array[1]);
  // println!("ArrayItem {}", int_array[3]); // Won't compile

  let int_vec = vec![1, 2, 3];
  // int_vec.push(4); // Won't compile because it's readonly
  println!("VectItem {}", int_vec[1]);
  // println!("VectItem {}", int_vec[3]); // will compile, but case a panic
  match int_vec.get(3) {
    Some(value) => println!("The value is {:?}", value),
    None => println!("There is no value for index 3"),
  }
  println!("VectItem {:?}", int_vec.get(2).unwrap());
  // println!("VectItem {:?}", int_vec.get(3).unwrap()); // Will panic
  println!("VectItem {:?}", int_vec.get(3).unwrap_or(&0));

  let mut mut_int_vec = vec![1, 2, 3];
  mut_int_vec.push(4);
}
