fn gen_random_vector() -> Vec<i32> {
    vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22]
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_indices = (0, 1);
    
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indices = (i, i + 1);
        }
    }
    
    (min_sum, min_indices.0, min_indices.1)
}

fn format_indexes(len: usize) -> String {
    (0..len).map(|i| format!("{:>3}", i)).collect::<Vec<_>>().join(". ")
}

fn format_data(data: &[i32]) -> String {
    data.iter().map(|x| format!("{:>2}", x)).collect::<Vec<_>>().join(" ")
}

fn display_result(data: &[i32]) {
    let (min_sum, index1, index2) = min_adjacent_sum(data);

    println!("indexes: {}", format_indexes(data.len()));
    println!("data:    {}", format_data(data));
    
    let mut indexes_line = vec![" "; data.len() * 4];
    indexes_line[index1 * 4 + 1] = "\\";
    indexes_line[index1 * 4 + 2] = "_";
    indexes_line[index1 * 4 + 3] = "_";
    indexes_line[index2 * 4 + 1] = "/";

    println!("indexes:{}", indexes_line.join(""));
    println!("min adjacent sum={} at indexes:{} and {}\n", min_sum, index1, index2);
}

fn main() {
    let data = gen_random_vector();
    display_result(&data);
}
