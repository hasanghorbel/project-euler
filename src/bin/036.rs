fn is_palindrome(n: u64, base: u64) -> bool {
    let mut reversed = 0;
    let mut k = n;
    while k > 0 {
        reversed = base * reversed + k % base;
        k /= base;
    }
    n == reversed
}

fn double_base_palindromes_i(n: u64) -> u64 {
    // It is easy to see that a number in base 2 must be odd to be a palindrome.
    let mut i = 1;
    let mut sum = 0;
    while i < n {
        if is_palindrome(i, 10) && is_palindrome(i, 2) {
            sum += i as u64;
        }
        i += 2;
    }
    sum
}

#[allow(dead_code)]
// this function is never used but good to know
fn make_palindrome(mut n: u64, base: u64, oddlength: bool) -> u64 {
    let mut result = n;
    if oddlength {
        n /= base;
    }
    while n > 0 {
        result = base * result + n % base;
        n /= base;
    }
    result
}

// better performance to generate palindromes in base 2
fn make_palindrome_base2(mut n: u64, oddlength: bool) -> u64 {
    let mut result = n;
    if oddlength {
        n = n >> 1;
    }
    while n > 0 {
        result = (result << 1) + (n & 1);
        n = n >> 1;
    }
    result
}

fn double_base_palindromes_ii(n: u64) -> u64 {
    let mut sum = 0;
    let mut i = 1;
    let mut p = make_palindrome_base2(i, true);
    while p < n {
        if is_palindrome(p, 10) {
            sum += p;
        }
        i += 1;
        p = make_palindrome_base2(i, true);
    }
    i = 1;
    p = make_palindrome_base2(i, false);
    while p < n {
        if is_palindrome(p, 10) {
            sum += p;
        }
        i += 1;
        p = make_palindrome_base2(i, false);
    }
    sum
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = double_base_palindromes_i(1_000_000);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let result = double_base_palindromes_ii(1_000_000);
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_palindrome(585, 2), true);
        assert_eq!(double_base_palindromes_i(1_000_000), 872187);
    }

    #[test]
    fn test_2() {
        println!("{}", make_palindrome(585, 10, false));
        assert_eq!(is_palindrome(585, 2), true);
        assert_eq!(double_base_palindromes_ii(1_000_000), 872187);
    }
}
