use std::collections::HashSet;

use num::BigUint;

fn distinct_powers(n: u32) -> usize {
  let mut set = HashSet::new();
  for i in 2..=n {
    for j in 2..=n {
      set.insert(BigUint::from(i).pow(j));
    }
  }

  set.len()
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  let result = distinct_powers(100);
  let elapsed = now.elapsed();
  println!("{}", result);
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(distinct_powers(5), 15);
  }
}