use crate::ChecksumOutput;
use std::fmt::Debug;

#[derive(PartialEq)]
pub struct ChecksumError {
	string: String,
	expected: ChecksumOutput,
	found: ChecksumOutput,
}

impl ChecksumError {
	pub fn new(string: String, expected: ChecksumOutput, found: ChecksumOutput) -> Self {
		Self { string, expected, found }
	}
}

impl<'a> Debug for ChecksumError {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		let found_str = &self.found.to_string();
		let expected_str = &self.expected.to_string();

		write!(
			fmt,
			"Wrong checksum for {}. Got {:?}={}, expected {:?}={}",
			self.string, self.found, found_str, self.expected, expected_str,
		)
	}
}
