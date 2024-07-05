fn sum_square_diff(n: u64) -> u64 {
  (n * (n + 1) * (3 * n * n - n - 2)) / 12
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", sum_square_diff(100));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_1() {
    assert_eq!(sum_square_diff(100), 25164150);
  }
}