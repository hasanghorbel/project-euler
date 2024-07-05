fn highly_divisible_triangular_num(n: u32) -> u32 {
  let mut cnt = 1; // counter
  let mut sum = 1; // triangular number
  let mut k = 0; // number of divisors
  while k < n
  {
    k = 0; 
    let mut x = sum;
    let mut p = 2; // prime divisor
    let mut i; // exponent
    while x != 1
    {
      i = 0;
      while x % p == 0
      {
          x = x / p;
          i+= 1;
      }
      k += i + i*k;
      p+= 1;
    }
    k+= 1;
    cnt+= 1;
    sum += cnt;
  }
  return sum - cnt;
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", highly_divisible_triangular_num(500));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(highly_divisible_triangular_num(500), 76576500);
  }
}