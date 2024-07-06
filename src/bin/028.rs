fn number_spiral_diagonals(n: u64) -> u64 {
  let n = (n - 1) / 2;
  1 + 4 * n + 2 * n * (n + 1) + 8 * n * (n + 1) * (2*n + 1) / 3
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  let result = number_spiral_diagonals(1001);
  let elapsed = now.elapsed();
  println!("{}", result);
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(number_spiral_diagonals(5), 101);
  }
}