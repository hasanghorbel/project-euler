fn function() {

}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  //let result = function();
  let elapsed = now.elapsed();
  //println!("{}", result);
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    //assert_eq!(function(), );
  }
}