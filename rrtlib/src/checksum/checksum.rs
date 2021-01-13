use crate::checksum::checksum_v00::ChecksumV00;
use crate::types::version::Version;
use std::str::FromStr;

pub trait RRTChecksum {
    // fn calculate<T>(s: T) -> u8;
    // fn calculate(s: &str) -> T; // TODO: remove that
    fn calculate(&mut self, s: &[u8]) -> [u8; 2];

    // TODO: I would prefer each Algo to define their return value, V00 -> u8, V01, [u8; 2]

    /// Used internally by is_valid(...)
    fn verify(&self, data: &[u8], checksum: u8) -> bool;

    /// Runs various tests to check whether a token is valid or not.
    /// This function does NOT verify the checksum.
    fn is_valid(&self, s: &[u8]) -> bool;

    // fn size_of(&self) -> usize;
}

pub enum Algo {
    V00(ChecksumV00),
    // ChecksumV01,
}

/// Given a u8 buffer, this function fetches the version
/// and returns the associated Algo
pub fn get_algo(s: &str) -> Option<Algo> {
    let version = Version::from_str(s);

    assert_eq!(version, Ok(Version::V00));

    Some(Algo::V00(ChecksumV00::new())) // TODO: pwaaa... that does not look great
}
