use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

fn parse(file: File) -> Result<Vec<Vec<u32>>, ParseIntError> {
    BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.is_empty())
        .map(|line| line.split_ascii_whitespace().map(|s| s.parse()).collect())
        .collect()
}

fn maximum_path_sum_ii(input: &[Vec<u32>]) -> u32 {
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

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    println!(
        "{}",
        maximum_path_sum_ii(&parse(File::open("data/triangle.txt").unwrap()).unwrap())
    );
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            maximum_path_sum_ii(&parse(File::open("data/triangle.txt").unwrap()).unwrap()),
            7273
        );
    }
}
