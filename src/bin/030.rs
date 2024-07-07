fn sum_digits(x: u32, exp: u32) -> u32 {
    let mut sum = 0;
    for i in x.to_string().chars() {
        sum += i.to_digit(10).unwrap().pow(exp);
    }
    sum
}

fn digit_fifth_sum(n: u32) -> u32 {
    // 6 * 9^5 = 354294
    // 7 * 9^5 = 413343
    // than we can take 354294 as our max
    (2..354294).filter(|k| sum_digits(*k, n) == *k).sum()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = digit_fifth_sum(5);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(digit_fifth_sum(4), 19316);
    }
}
