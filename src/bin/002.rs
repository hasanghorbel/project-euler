fn even_fibonacci_numbers(n: f64) -> u64 {
    // maximum index for Fibonacci sequence
    let n = ((n.log(10_f64) + 5_f64.sqrt().log(10_f64))
        / ((1_f64 + 5_f64.sqrt()) / 2_f64).log(10_f64))
    .floor();

    // formula for the sum using binet's formula
    let phi = ((1_f64 + 5_f64.sqrt()) / 2_f64).powf(n + 2_f64);
    let psi = ((1_f64 - 5_f64.sqrt()) / 2_f64).powf(n + 2_f64);
    return (((phi - psi) / 5_f64.sqrt()) as u64 - 1) / 2;
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = even_fibonacci_numbers(4_000_000_f64);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(even_fibonacci_numbers(4_000_000_f64), 4613732);
    }
}
