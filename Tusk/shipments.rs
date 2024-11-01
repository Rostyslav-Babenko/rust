fn min_moves(weights: &[u32]) -> isize {
	let n = weights.len();
	let total: u32 = weights.iter().sum();

	if total as usize % n != 0 {
    	return -1;
	}

	let avg = total / n as u32;
	let mut moves = 0;
	let mut balance = 0;

	for &weight in weights {
    	balance += weight as isize - avg as isize;
    	moves += balance.abs();
	}

	moves as isize
}

fn generate_weights(n: usize) -> Vec<u32> {
	vec![10; n]
}

fn main() {
	let weights = vec![8, 2, 2, 4, 4];
	println!("Moves needed: {}", min_moves(&weights));

	let generated = generate_weights(5);
	println!("Generated weights: {:?}", generated);

	let other_weights = vec![9, 3, 7, 2, 9];
	println!("Moves for another case: {}", min_moves(&other_weights));
}
