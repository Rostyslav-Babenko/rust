use std::io::{self, BufRead};

fn birthday(s: Vec<i32>, d: i32, m: i32) -> i32 {
    let n = s.len();
    let mut count = 0;

    for i in 0..(n - m as usize + 1) {
        let sum: i32 = s[i..(i + m as usize)].iter().sum();
        if sum == d {
            count += 1;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let input_lines: Vec<String> = handle.lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let n: usize = input_lines[0].trim().parse().expect("Invalid number");
    let s: Vec<i32> = input_lines[1]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let d_m: Vec<i32> = input_lines[2]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let d = d_m[0];
    let m = d_m[1];

    let result = birthday(s, d, m);
    println!("{}", result);
}