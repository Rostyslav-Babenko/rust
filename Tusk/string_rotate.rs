fn rotate(s: String, n: isize) -> String {
	let len = s.len() as isize;
	if len == 0 {
    	return s;
	}

	let shift = ((n % len) + len) % len;
	let split = (len - shift) as usize;

	s[split..].to_string() + &s[..split]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
    	let s = "abcdefgh";
    	let shifts = [
        	(0, "abcdefgh"),
        	(8, "abcdefgh"),
        	(-8, "abcdefgh"),
        	(1, "habcdefg"),
        	(2, "ghabcdef"),
        	(10, "ghabcdef"),
        	(-1, "bcdefgha"),
        	(-2, "cdefghab"),
        	(-10, "cdefghab"),
    	];

    	shifts.iter().for_each(|(n, exp)| {
        	assert_eq!(rotate(s.to_string(), *n), exp.to_string());
    	});
	}
}

fn main() {
	let s = "abcdefgh".to_string();
	let rotated = rotate(s, 2);
	println!("Rotated string: {}", rotated);
}
