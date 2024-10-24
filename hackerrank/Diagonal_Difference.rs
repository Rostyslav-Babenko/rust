use std::io;

fn diagonal_difference(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += matrix[i][i];
        secondary_diagonal_sum += matrix[i][n - i - 1];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut matrix: Vec<Vec<i32>> = Vec::new(); 

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix.push(row);
    }

    let result = diagonal_difference(matrix);
    println!("{}", result);
}