use std::io::{self, BufRead};

fn mini_max_sum(arr: &[u64]) {
    let total_sum: u64 = arr.iter().sum();
    let min_sum = total_sum - arr.iter().max().unwrap();
    let max_sum = total_sum - arr.iter().min().unwrap();

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let arr: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        mini_max_sum(&arr);
    }
}