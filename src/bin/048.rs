use num::{BigUint, Zero};

fn self_powers(n: u32) -> u64 {
    let mut sum = BigUint::zero();
    for i in 1..=n {
        sum += BigUint::from(i).pow(i);
        sum = sum % "10_000_000_000".parse::<BigUint>().unwrap();
    }
    sum.try_into().unwrap()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = self_powers(1000);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(self_powers(10), 405071317);
    }
}
