fn multiples_of_3_or_5(n: u32) -> u32 {
  (1..n).filter(|&k| k % 3 == 0 || k % 5 == 0).sum()
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", multiples_of_3_or_5(1000));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_1() {
    assert_eq!(multiples_of_3_or_5(1000), 233168);
  }
}