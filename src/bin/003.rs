fn largest_prime_factor(mut n: u64) -> u64 {
  let mut factor = 2;
  while factor * factor <= n {
    while n % factor == 0 && n != factor {
      n /= factor;
    }
    factor+= 1;
  }
  return n;
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}",largest_prime_factor(600851475143));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
  fn test_1() {
    assert_eq!(largest_prime_factor(600851475143), 6857);
  }
}