fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| if c.is_lowercase() { c.to_uppercase().to_string() } 
                 else { c.to_lowercase().to_string() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        data.iter().for_each(|(a, b)| {
            assert_eq!(invert_the_case(a.to_string()), b.to_string());
            assert_eq!(invert_the_case(b.to_string()), a.to_string());
        });
    }
}

fn main() {
    let text = "Hello, Привет!";
    let swapped_text = invert_the_case(text.to_string());
    println!("Original: {}", text);
    println!("Swapped: {}", swapped_text);
}