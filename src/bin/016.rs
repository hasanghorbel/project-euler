fn power_digit_sum(n: u32, exp: usize) -> u32 {
  let mut cache = vec![];
  cache.push(vec![1]);
  for _ in cache.len()..=exp {
    let mut power = cache[cache.len() - 1].clone();
    let mut carry = 0;
    for i in 0..power.len() {
      power[i] = n * power[i] + carry;

      if power[i] >= 10 {
        power[i] -= 10;
        carry = 1;
      } else {
        carry = 0;
      }
    }
    if carry != 0 {
      power.push(carry);
    }
    cache.push(power)
  }

  // sum of all digits
  let mut sum = 0;
  for i in &cache[exp] {
    sum += i;
  }
  sum
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", power_digit_sum(2, 1000));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(power_digit_sum(2, 1000), 1366);
  }
}