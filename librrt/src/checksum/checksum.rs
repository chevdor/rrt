use std::fmt::Display;

pub trait Checksum<T: PartialEq> {
	fn calculate(&self, s: &[u8]) -> T;

	/// Verifies whether or not the checksum of `data` is `checksum`.
	fn verify(&self, data: &[u8], checksum: T) -> bool {
		self.calculate(data) == checksum
	}

	fn checksum(&self) -> Option<T>;
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ChecksumOutput {
	Single(u8),
	Dual([u8; 2]),
}

impl Into<Vec<u8>> for ChecksumOutput {
	fn into(self) -> Vec<u8> {
		match self {
			ChecksumOutput::Single(s) => vec![s],
			ChecksumOutput::Dual(d) => d.into(),
		}
	}
}

impl Display for ChecksumOutput {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		let string = match self {
			ChecksumOutput::Single(s) => String::from(*s as char),
			ChecksumOutput::Dual(d) => String::from_utf8_lossy(d).to_string(),
		};
		write!(fmt, "{}", string)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_convert_checksum_output() {
		let s1 = ChecksumOutput::Single(65);
		let s2 = ChecksumOutput::Dual([65, 66]);

		let r1: Vec<u8> = s1.into();
		let r2: Vec<u8> = s2.into();
		assert_eq!(vec![65], r1);
		assert_eq!(vec![65, 66], r2);
	}
}
