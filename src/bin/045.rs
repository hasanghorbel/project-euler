use num::integer::Roots;

fn is_pentagonal(n: u64) -> bool {
  let s = (24 * n + 1).sqrt();
  (s + 1) % 6 == 0 && s * s == 1 + 24 * n
}

fn triangular_pentagonal_and_hexagonal() -> u64 {
  let mut m = 144;
  loop {
    let result = 2 * m *m -m;
    if is_pentagonal(result) {
      return result;
    }
    m += 1;
  }
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  let result = triangular_pentagonal_and_hexagonal();
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
    assert_eq!(is_pentagonal(22), true);
  }
}