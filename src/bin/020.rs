fn factorial_digit_sum(n: u32) -> u32 {
  let mut cache = vec![0; (n*2).try_into().unwrap()];
  cache[0] = 1;

  for i in 1..=n {
    let mut carry = 0;
    for j in 0..cache.len() {
      let mut digit = i * cache[j];
      if carry > 0 {
        digit += carry;
        carry = 0;
      }
      while digit >= 10 {
          digit -= 10;
          carry += 1;
      }
      cache[j] = digit;
    }
  }

  cache.iter().sum()
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", factorial_digit_sum(100));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(factorial_digit_sum(100), 648);
  }
}