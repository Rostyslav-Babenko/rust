use itertools::Itertools;

fn main() {
    let numbers = (1..=8).collect::<Vec<_>>();  // Створюємо вектор з чисел від 1 до 8
    let permutations = numbers.into_iter().permutations(8);  // Генерація перестановок

    for perm in permutations {
        let (m, u, x, a, s, l, o, n) = (perm[0], perm[1], perm[2], perm[3], perm[4], perm[5], perm[6], perm[7]);
        
        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        println!("muxa: {}, a: {}, slon: {}", muxa, a, slon);  // Виведення для відладки

        if muxa * a == slon {
            println!("{:04} x {} = {:04}", muxa, a, slon);
        }
    }
}
