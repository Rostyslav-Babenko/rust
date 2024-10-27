use std::io::{self, BufRead};

fn plus_minus(arr: &[i32]) {
    let total = arr.len() as f64;
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;

    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    println!("{:.6}", positive_count as f64 / total);
    println!("{:.6}", negative_count as f64 / total);
    println!("{:.6}", zero_count as f64 / total);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    lines.next();
    
    if let Some(Ok(line)) = lines.next() {
        let arr: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        plus_minus(&arr);
    }
}
