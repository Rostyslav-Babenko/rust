use std::io;

fn bon_appetit(bill: Vec<i32>, k: usize, b_charged: i32) {

    let total_cost: i32 = bill.iter().sum();
    let anna_share = (total_cost - bill[k]) / 2;

    if b_charged == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b_charged - anna_share);
    }
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nk: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();
    let k = nk[1];
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let bill: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b_charged: i32 = input.trim().parse().expect("Parse error");
    
    bon_appetit(bill, k, b_charged);
}

