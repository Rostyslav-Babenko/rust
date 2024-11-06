use std::collections::HashMap;
use std::io;

fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut bird_count = HashMap::new();

    for bird in arr {
        *bird_count.entry(bird).or_insert(0) += 1;
    }

    let mut max_bird = -1;
    let mut max_count = 0;
    
    for (&bird_type, &count) in &bird_count {
        if count > max_count || (count == max_count && bird_type < max_bird) {
            max_bird = bird_type;
            max_count = count;
        }
    }
    
    max_bird
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();
    
    println!("{}", migratory_birds(arr));
}
