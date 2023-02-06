use std::fmt;

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Point x={}, y={}", self.x, self.y)
  }
}

fn main() {
  let p = Point { x: 5, y: 5 };
  println!(" {:?}", p);
}
