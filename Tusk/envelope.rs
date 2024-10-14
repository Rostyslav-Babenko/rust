const WIDTH: usize = 11;
const HEIGHT: usize = 11;

fn main() {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            print!("{}", if row == 0 || row == HEIGHT - 1 || col == 0 || col == WIDTH - 1 || row == col || row == HEIGHT - col - 1 { "*" } else { " " });
        }
        println!();
    }
}
