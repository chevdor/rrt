use crate::checksum::checksum::Checksum;
use crate::error::ChecksumError;
use std::str::FromStr;

/// This is simple checksum that is not robust agains position swap. It is mainly
/// in there because it is simple and good for testing.
#[derive(Debug, Default)]
pub struct ChecksumV00 {
	checksum: Option<u8>,
}

impl ChecksumV00 {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Checksum<u8> for ChecksumV00 {
	/// We want a checksum being a value 65..90
	/// So we take the modulo 26 and shift to the first char.
	fn calculate(&self, s: &[u8]) -> u8 {
		let sum: u8 = s.iter().fold(0, |s, &x| s.wrapping_add(x));
		sum % 26 + 65
	}

	/// Getter
	fn checksum(&self) -> Option<u8> {
		self.checksum
	}
}

impl FromStr for ChecksumV00 {
	type Err = ChecksumError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let checksum = ChecksumV00::new();
		let c = checksum.calculate(s.as_bytes());
		Ok(Self { checksum: Some(c) })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_calculates() {
		let checksum = ChecksumV00::new();
		assert_eq!(checksum.calculate("A".as_bytes()), 78);
	}

	#[test]
	fn it_calculate() {
		let checksum = ChecksumV00::new();
		assert_eq!(checksum.calculate(b"A"), 78);
	}

	#[test]
	fn it_calculates_from_str() {
		let checksum = ChecksumV00::new();

		assert_eq!(checksum.calculate(b"0"), 87);
		assert_eq!(checksum.calculate(b"1"), 88);
		assert_eq!(checksum.calculate(b"A"), 78);
		assert_eq!(checksum.calculate(b"AA"), 65);
		assert_eq!(checksum.calculate(b"AB06"), 90);
		assert_eq!(checksum.calculate(b"AB07"), 65);
		assert_eq!(checksum.calculate(b"AB09"), 67);
		assert_eq!(checksum.calculate(b"ZZZZZZ"), 67);
		assert_eq!(checksum.calculate(b"010012345TWBABAEFGH"), 74);
	}

	#[test]
	fn it_has_a_checksum_always_between_65_and_90() {
		let checksum = ChecksumV00::new();
		let mut s = String::new();

		for _ in 1..26 * 100 {
			s += "A";
			let c = checksum.calculate(s.as_bytes());
			assert!(c >= 65); // Ascii A
			assert!(c <= 90); // Ascii Z
		}
	}

	#[test]
	#[ignore = "Does not work for this checksum"]
	fn it_prevents_typical_swaps() {
		let checksum = ChecksumV00::new();

		assert!(checksum.calculate("AB".as_bytes()) != checksum.calculate("BA".as_bytes()));
	}
}
