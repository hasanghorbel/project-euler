#[allow(dead_code)]

fn a_bold_proposition() {
    // notice that 57 is a prime
    // btw you need a phd to solve this problem
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
