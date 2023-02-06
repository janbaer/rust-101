use std::thread;

fn main() {
  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || {
    println!("Hier ist ein Vektor: {:?}", v);
  });

  handle.join().unwrap();
}
