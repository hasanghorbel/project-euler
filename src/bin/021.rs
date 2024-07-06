fn amicable_numbers(n: usize) -> usize {
    let mut cache = vec![0; n];
    let mut sum = 0;
    for a in 2..n {
        // d(a)
        let mut sum_a;
        if cache[a] == 0 {
            sum_a = 0;
            for i in 1..=a / 2 {
                if a % i == 0 {
                    sum_a += i;
                }
            }
            cache[a] = sum_a;
        } else {
            sum_a = cache[a];
        }

        // d(b)
        let mut sum_b = 0;
        if sum_a < n && cache[sum_a] == 0 {
            sum_b = 0;
            for i in 1..=sum_a / 2 {
                if sum_a % i == 0 {
                    sum_b += i;
                }
            }
            cache[sum_a] = sum_b;
        } else if sum_a < n && cache[sum_a] != 0 {
            sum_b = cache[sum_a];
        }

        if sum_b == a && a != sum_a {
            sum += a;
        }
    }

    return sum;
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = amicable_numbers(10000);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(amicable_numbers(10000), 31626);
    }
}
