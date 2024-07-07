use primes::{Sieve, PrimeSet};

fn is_prime(n: i32) -> bool {
  if n < 1 {
    return false;
  }
  let mut i = 2;
  while i * i <= n {
    if n % i == 0 {
      return false;
    }
    i += 1;
  }
  true
}

fn count_consecutive_primes(a: i32, b: i32) -> i32 {
  let mut n = 0;
  while is_prime(n * n + a * n + b) {
    n += 1;
  }
  n
}

fn quadratic_primes(bound: i32) -> i32 {
  let mut pset = vec![];
  for p in Sieve::new().iter() {
    if p > bound as u64 {
      break;
    }
    pset.push(p as i32);
  }
  
  let mut max_c = 0;
  let mut max_ab = 0;
  for a in (-999..bound).step_by(2) {
    for i in 0..pset.len() {
      let b = pset[i];
      let c = count_consecutive_primes(a, b);
      if c > max_c {
        max_c = c;
        max_ab = a * b;
      }
    }
  }
  max_ab
}



fn main() {
  use std::time::Instant;
  let now = Instant::now();
  let result = quadratic_primes(1000);
  let elapsed = now.elapsed();
  println!("{}", result);
  println!("Elapsed: {:.2?}", elapsed);
}
// 971 -61
#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(is_prime(19), true);
    assert_eq!(count_consecutive_primes(-61, 971), 71);
    assert_eq!(quadratic_primes(1000), -59231);
  }
}