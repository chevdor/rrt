use std::fmt::Debug;

pub struct ChecksumError {
    expected: u8,
    found: u8,
}

impl ChecksumError {
    pub fn new(expected: u8, found: u8) -> Self {
        Self { expected, found }
    }
}

pub type ChecksumErrorT = (u8, u8);

impl Debug for ChecksumError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Wrong checksum. Got {}={}, expected {}={}",
            self.found, self.found as char, self.expected, self.expected as char,
        )
    }
}
