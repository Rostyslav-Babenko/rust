use std::collections::HashMap;
use std::io;

fn sock_merchant(socks: Vec<i32>) -> i32 {
    let mut sock_counts = HashMap::new();
    for sock in socks {
        *sock_counts.entry(sock).or_insert(0) += 1;
    }
    sock_counts.values().map(|&count| count / 2).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: i32 = input.trim().parse().unwrap(); 

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let socks: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", sock_merchant(socks));
}
