fn main() {
    const SIZE: usize = 5; // Розмір ромба

    // Малюємо ромб
    for i in 0..(2 * SIZE) {
        let stars = if i < SIZE { 2 * i + 1 } else { 2 * (2 * SIZE - i - 1) + 1 };
        let spaces = SIZE as isize - stars as isize / 2 - 1;

        print!("{:width$}{}", "", "*".repeat(stars), width = spaces as usize);
        println!();
    }
}
