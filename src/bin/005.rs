fn smallest_multiple(n: u64) -> u64 {
  let mut result = 1;
  for i in 2..=n {
    result = num::integer::lcm(result, i);
  }
  return result;
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", smallest_multiple(20));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
  fn test_1() {
    assert_eq!(smallest_multiple(20), 232792560);
  }
}