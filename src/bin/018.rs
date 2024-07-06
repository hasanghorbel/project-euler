use std::{cmp::max, num::ParseIntError};

fn parse(s: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.split_ascii_whitespace().map(|s| s.parse()).collect())
        .collect()
}

fn maximum_path_sum_i(input: &[Vec<u32>]) -> u32 {
    let mut last = vec![input[0][0]];

    for row in 1..input.len() {
        let mut current = vec![];

        for column in 0..=row {
            let mut left_parent = 0;
            if column > 0 {
                left_parent = last[column - 1];
            }

            let mut right_parent = 0;
            if column < last.len() {
                right_parent = last[column];
            }

            let sum = input[row][column] + max(left_parent, right_parent);
            current.push(sum);
        }
        last = current.clone();
    }
    *last.iter().max().unwrap()
}

const TRIANGLE: &str = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = maximum_path_sum_i(&parse(TRIANGLE).unwrap());
    let elapsed = now.elapsed();
    println!("{}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(maximum_path_sum_i(&parse(TRIANGLE).unwrap()), 1074);
    }
}
