use std::cmp::Ordering;

fn gcd(a: i32, b: i32) -> i32 {
    match b.cmp(&0) {
        Ordering::Equal => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

fn lcm_array(arr: &[i32]) -> i32 {
    arr.iter().fold(1, |acc, &num| lcm(acc, num))
}

fn gcd_array(arr: &[i32]) -> i32 {
    arr.iter().fold(arr[0], |acc, &num| gcd(acc, num))
}

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = lcm_array(a);
    let gcd_b = gcd_array(b);
    
    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];
    let result = get_total_x(&a, &b);
    println!("{}", result);
}
