fn champernownes_constant(bound: usize) -> u32 {
    let mut s = String::new();
    let mut result = 1;
    // I made 190_000 the maximum for better performance
    // you need to change it for bigger numbers
    for i in 1..190_000 {
        s += &i.to_string();
    }
    let mut j = 1;
    while j <= bound {
        result *= (s.as_bytes()[j - 1] as char).to_digit(10).unwrap();
        j *= 10;
    }
    result
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = champernownes_constant(1_000_000);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(champernownes_constant(1_000_000), 210);
    }
}
