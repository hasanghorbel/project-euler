use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse(file: File) -> Vec<String> {
    let mut names = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(",")
                .map(|s| s[1..s.len() - 1].to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
        .concat();
    names.sort_by(|a, b| a.cmp(&b));
    names
}

fn score(name: &String) -> usize {
    name.chars().map(|c| c as usize - 64).sum()
}

fn names_score(names: &Vec<String>) -> usize {
    names
        .iter()
        .enumerate()
        .map(|(i, name)| (i + 1) * score(name))
        .sum()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = names_score(&parse(File::open("data/names.txt").unwrap()));
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
            names_score(&parse(File::open("data/names.txt").unwrap())),
            871198282
        );
    }
}
