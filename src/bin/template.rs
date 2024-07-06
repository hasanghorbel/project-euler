#[allow(dead_code)]

fn function() {

}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  //let result = function();
  let elapsed = now.elapsed();
  //println!("{}", function());
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::*;

  #[test]
  fn test_1() {
    //assert_eq!(function(), );
  }
}