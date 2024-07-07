fn is_prime(n: u32) -> bool {
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


fn circular_primes(n: u32) -> u32 {
  let mut counter = 0;
  for i in 2..n {
    if !is_prime(i) {
      continue;
    }
    let s = i.to_string();
    let mut prime = true;
    for j in 1..s.len() {
      if !is_prime(format!("{}{}", &s[j..], &s[..j]).parse::<u32>().unwrap()) {
        prime = false;
        break;
      }
    }
    if prime {
      counter += 1;
    }
  }
  counter
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  let result = circular_primes(1_000_000);
  let elapsed = now.elapsed();
  println!("{}", result);
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(circular_primes(100), 13);
  }
}