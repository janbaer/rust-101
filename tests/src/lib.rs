pub fn add(left: usize, right: usize) -> usize {
  left + right
}

fn minus(left: usize, right: usize) -> usize {
  left - right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_should_return_sum() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  fn minus_should_return_difference() {
    let result = minus(4, 2);
    assert_eq!(2, result);
  }
}
