fn make_palindrome(x: u64) -> u64 {
  let mut result = x * 1000; // abc => abc00
  result += x / 100;              // a.. => a..00a
  result += ((x / 10) % 10) * 10; // .b. => .b.0b.
  result += (x % 10) * 100;       // ..c => ..cc..
  return result;
}

fn largest_palindrome_product() -> u64{
  for i in (100..=999).rev() {
    let palindrome = make_palindrome(i);

    // split two factors
    let mut j = 99;
    while j*j <= palindrome {
      j += 1;
      if palindrome % j == 0 {
        let other = palindrome / j;
        if other < 100 || other > 999 {
          continue;
        }
        return palindrome;
      }
    }
  }
  return 0;
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", largest_palindrome_product());
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
  fn test_1() {
    assert_eq!(largest_palindrome_product(), 906609);
  }
}