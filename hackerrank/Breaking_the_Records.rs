use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let scores: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut highest_score = scores[0];
    let mut lowest_score = scores[0];
    let mut highest_breaks = 0;
    let mut lowest_breaks = 0;

    
    for &score in &scores[1..] {
        if score > highest_score {
            highest_score = score;
            highest_breaks += 1;
        } else if score < lowest_score {
            lowest_score = score;
            lowest_breaks += 1;
        }
    }


    println!("{} {}", highest_breaks, lowest_breaks);
}
