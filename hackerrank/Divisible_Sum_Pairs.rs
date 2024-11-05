use std::io::{self, BufRead};

fn divisible_sum_pairs(n: usize, k: i32, arr: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            if (arr[i] + arr[j]) % k == 0 {
                count += 1;
            }
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

    let first_line: Vec<i32> = input_lines[0]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let n = first_line[0] as usize; 
    let k = first_line[1]; 

    let arr: Vec<i32> = input_lines[1]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let result = divisible_sum_pairs(n, k, arr);
    println!("{}", result);
}
