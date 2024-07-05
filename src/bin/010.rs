fn summation_of_primes(lim: u64) -> u64 {
  let mut s = 2;
  let mut primes: Vec<u64> = vec![];
  for i in (3..lim).step_by(2) {
    let mut is_prime = true;
    for p in &primes {
      // first check
      if i % p == 0 {
        is_prime = false;
        break;
      }
      // second check
      if p*p > i {
        break;
      }
    }
    if is_prime {
      s += i;
      primes.push(i);
    }
  }
  return s;
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", summation_of_primes(2_000_000));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(summation_of_primes(2_000_000), 142913828922)
  }
}