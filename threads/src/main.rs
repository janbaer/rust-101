use std::thread;
use std::time::Duration;

fn main() {
  let _handle = thread::spawn(|| {
    for i in 1..10 {
      println!("Hallo {} thread welt", i);
      thread::sleep(Duration::from_millis(10));
    }
  });

  // _handle.join().unwrap();

  for i in 1..5 {
    println!("Hallo {} welt", i);
    thread::sleep(Duration::from_millis(5));
  }

  // _handle.join().unwrap();
}
