fn n_th_prime(n: usize) -> u64 {
    let mut primes: Vec<u64> = vec![];
    primes.reserve(n);
    primes.push(2);
    let mut i = 3;
    while primes.len() < n {
        let mut is_prime = true;
        for p in 0..primes.len() {
            // first check
            if i % primes[p] == 0 {
                is_prime = false;
                break;
            }

            // second check
            if primes[p] * primes[p] > i {
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
        i += 2;
    }
    return primes[n - 1];
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = n_th_prime(10001);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(n_th_prime(10001), 104743);
    }
}
