use std::io::{self, BufRead};

fn staircase(n: usize) {
    for i in 1..=n {
        let spaces = " ".repeat(n - i);
        let hashes = "#".repeat(i);
        println!("{}{}", spaces, hashes);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let n: usize = line.trim().parse().unwrap();
        staircase(n);
    }
}