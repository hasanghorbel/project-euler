fn fact(n: u32) -> u32 {
  (1..=n).product()
}


fn sum_digits(mut n: usize, facts: &Vec<u32>) ->  usize {
  let mut sum= 0;
  while n != 0 {
    sum += facts[n % 10] as usize;
    n = n / 10;
  }
  sum
}


fn digit_factorials() -> usize {
  // notice that 7 * 9! > 2540160
  // we take 2540160 as a maximum
  let facts  = (0..10).map(|n| fact(n)).collect::<Vec<u32>>();
  (3..2540160).filter(|n| sum_digits(*n, &facts) == *n).sum()
  
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  let result = digit_factorials();
  let elapsed = now.elapsed();
  println!("{}", result);
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(digit_factorials(), 40730);
  }
}