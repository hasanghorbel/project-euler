use num::One;
use num_bigint::BigUint;

fn nth_digit_fibonacci_number(n: usize) -> usize {
    let mut a = BigUint::one();
    let mut b = BigUint::one();
    let mut idx = 2;
    while a <= "9".repeat(n - 1).parse::<BigUint>().unwrap() {
        let c = a.clone();
        a += &b;
        b = c;
        idx += 1;
    }
    idx
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = nth_digit_fibonacci_number(1000);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(nth_digit_fibonacci_number(3), 12);
    }
}
