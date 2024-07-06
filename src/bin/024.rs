fn fact(n: usize) -> usize {
    (1..=n).product()
}

fn lexicographic_permutations(n: usize, mut digits: Vec<usize>) -> usize {
    let mut n = n - 1;
    let mut result = 0;
    let mut exp = 10_usize.pow((digits.len() - 1) as u32);
    for i in (0..digits.len()).rev() {
        let i_fact = fact(i);
        let permutation = n / i_fact;
        result += digits[permutation] * exp;
        exp /= 10;
        n = n - permutation * i_fact;
        digits.remove(permutation);
    }
    result
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = lexicographic_permutations(1_000_000, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            lexicographic_permutations(1_000_000, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            2783915460
        );
    }
}
