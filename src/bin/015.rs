use num::integer::gcd;

fn lattice_paths(m: u64, n: u64) -> u64 {
    let mut n_fact = 1;
    let mut mn_fact = 1;

    for i in 1..=n {
        n_fact *= i;
        mn_fact *= i + m;
        let g = gcd(n_fact, mn_fact);
        n_fact /= g;
        mn_fact /= g;
    }

    return mn_fact / n_fact;
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = lattice_paths(20, 20);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(lattice_paths(20, 20), 137846528820);
    }
}
