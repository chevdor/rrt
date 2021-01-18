use std::fmt::Debug;

#[derive(PartialEq)]
pub struct ChecksumError {
    string: String,
    expected: u8,
    found: u8,
}

impl ChecksumError {
    pub fn new(string: String, expected: u8, found: u8) -> Self {
        Self {
            string,
            expected,
            found,
        }
    }
}

impl Debug for ChecksumError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Wrong checksum for {}. Got {}={}, expected {}={}",
            self.string, self.found, self.found as char, self.expected, self.expected as char,
        )
    }
}
