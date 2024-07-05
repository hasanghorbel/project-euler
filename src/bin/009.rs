use std::iter;
use num::integer::gcd;

fn special_pythagorean_triplet_1(sum: u32) -> u32 {
    (1..sum)
        .flat_map(|a| (a + 1..500 - a / 2).zip(iter::repeat(a)))
        .map(|(a, b)| (a, b, sum - a - b))
        .find(|&(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

fn special_pythagorean_triplet_2(sum: u32) -> u32 {
  let s2 = sum / 2;
  let mlim = (s2 as f32).sqrt().ceil() as u32 - 1;
  for m in 2..mlim {
    if s2 % m == 0 {
      let mut sm = s2 / m;
      while sm % 2 == 0 {
        sm = sm / 2;
      }
      let mut k;
      if m % 2 == 1 {
        k = m + 2;
      } else {
        k = m + 1;
      }

      while k < 2 * m && k <= sm {
        if sm % k == 0 && gcd(k, m) == 1 {
          let d = s2 / (k*m);
          let n = k - m;
          let a = d * (m * m - n * n);
          let b = 2 * d * m * n;
          let c = d * (m * m + n * n);
          return a * b* c;
        }
        k += 2;
      }
    }
  }
  return 0;
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    println!("{}", special_pythagorean_triplet_1(1000));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    println!("{}", special_pythagorean_triplet_2(1000));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(special_pythagorean_triplet_1(1000), 31875000)
    }
    
    #[test]
    fn test_2() {
      assert_eq!(special_pythagorean_triplet_2(1000), 31875000)
  }
}
