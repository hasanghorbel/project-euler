fn counting_sundays(start: u32, finish: u32) -> u32 {
  let mut dm = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  let mut result = 0;
  let mut sunday = 6; // date of sunday in the month
  for year in start..finish {
    if year % 4 == 0 {
      dm[1] = 29;
    }
    else if year % 4 == 1 {
      dm[1] = 28;
    }
    let mut month = 0; // january starts at 0
    while month < 12 {
      sunday += 7;
      if sunday > dm[month] {
        sunday -= dm[month];
        month+=1;
        if sunday == 1 {
          result += 1;
        }
      }
    }
  }
  // edge case
  if sunday == 1 {
    result -= 1;
  }
  result
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  println!("{}", counting_sundays(1901, 2001));
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(counting_sundays(1901, 2001), 171);
  }
}