fn count_fruits(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {
    let apples_on_house = apples.iter()
        .map(|&d| a + d)
        .filter(|&position| position >= s && position <= t)
        .count() as i32;

    let oranges_on_house = oranges.iter()
        .map(|&d| b + d)
        .filter(|&position| position >= s && position <= t)
        .count() as i32;

    (apples_on_house, oranges_on_house)
}

fn main() {
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    let (apples_count, oranges_count) = count_fruits(s, t, a, b, &apples, &oranges);
    
    println!("{}", apples_count);
    println!("{}", oranges_count); 
}
