use std::{fs::File, io::{BufRead, BufReader}};

fn parse(file: File) -> Vec<String> {
  let mut names = BufReader::new(file)
      .lines()
      .map_while(Result::ok)
      .filter(|l| !l.is_empty())
      .map(|line| {
          line.split(",")
              .map(|s| s[1..s.len() - 1].to_string())
              .collect::<Vec<String>>()
      })
      .collect::<Vec<Vec<String>>>()
      .concat();
  names.sort_by(|a, b| a.cmp(&b));
  names
}

fn score(name: &String) -> u64 {
  name.chars().map(|c| c as u64 - 64).sum()
}

fn is_triangle(x: u64) -> bool {
  let mut n = 1;
  while n * (n+1) / 2 < x {
    n += 1;
  }
  n * (n+1) / 2 == x
}

fn coded_triangle_numbers(input: Vec<String>) -> u64 {
  let mut counter = 0;
  for s in input {
    if is_triangle(score(&s)) {
      counter += 1
    }
  }
  counter
}

fn main() {
  use std::time::Instant;
  let now = Instant::now();
  let result = coded_triangle_numbers(parse(File::open("data/words.txt").unwrap()));
  let elapsed = now.elapsed();
  println!("{}", result);
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(is_triangle(28), true);
    assert_eq!(score(&"SKY".to_owned()), 55);
  }
}