use std::collections::HashMap;

fn steps(map: &mut HashMap<u64, u64>, n: u64) -> u64 {
  if let Some(&num) = map.get(&n) {
    return num;
  }

  let next;
  if n % 2 == 0 {
    next = n / 2;
  } else {
    next = 3 * n + 1;
  }

  let result = 1 + steps(map, next);
  let _ = map.insert(n, result);
  result
}

fn longest_collatz_sequence(limit: u64) -> u64 {
  let mut map = HashMap::with_capacity(limit as usize);
  let _ = map.insert(1, 1);

  (2..limit)
      .max_by_key(|&n| steps(&mut map, n))
      .unwrap()
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", longest_collatz_sequence(1_000_000));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(longest_collatz_sequence(1_000_000), 837799);
  }
}