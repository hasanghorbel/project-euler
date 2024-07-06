fn is_abundant_num(n: &usize) -> bool {
    &(1..=n / 2).filter(|d| n % d == 0).sum::<usize>() > n
}

fn non_abundant_sums(bound: usize) -> usize {
    let abundant = (2..bound + 1)
        .filter(|&n| is_abundant_num(&n))
        .collect::<Vec<usize>>();
    let mut abundant_sums = vec![false; 2 * bound + 1];
    for i in 0..abundant.len() {
        for j in i..abundant.len() {
            abundant_sums[abundant[i] + abundant[j]] = true;
        }
    }
    let sum = (1..bound + 1).filter(|&i| !abundant_sums[i]).sum();
    sum
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = non_abundant_sums(28123);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        //println!("{}", is_abundant_num(&28));
        println!("{:?}", non_abundant_sums(28123));
    }
}
