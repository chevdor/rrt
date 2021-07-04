use rand::Rng;
use std::fmt::UpperHex;
use std::str;

pub fn dec2hex<T: UpperHex>(x: T, width: usize) -> String {
	format!("{:0width$X}", x, width = width)
}

/// Generate a random string of 'length' chars
/// The string is made of ascii chars from 65 to 90 (CAPS).
pub fn gen_random_string(length: usize) -> String {
	let mut rng = rand::thread_rng();
	let chars: Vec<u8> = (0..length).map(|_| rng.gen_range(65..=90)).collect();
	String::from(str::from_utf8(&chars).unwrap())
}

/// This function removes any char that is not part of [A-Z0-9]
pub fn clean_token_string(s: &str) -> String {
	s.chars().filter(|c| (*c >= 'A' && *c <= 'Z') || (*c >= '0' && *c <= '9')).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_dec2hex() {
		assert_eq!(dec2hex(0, 2), "00");
		assert_eq!(dec2hex(0, 4), "0000");
		assert_eq!(dec2hex(10, 2), "0A");
		assert_eq!(dec2hex(15, 2), "0F");
		assert_eq!(dec2hex(16, 2), "10");
		assert_eq!(dec2hex(255, 2), "FF");
		assert_eq!(dec2hex(11041, 5), "02B21");
	}

	#[test]
	fn it_generate_a_8_chars_token() {
		let token = &gen_random_string(8);
		assert_eq!(token.len(), 8);
	}
}
