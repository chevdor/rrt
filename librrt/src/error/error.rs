use crate::error::checksum_error::ChecksumError;
use crate::types::VersionError;

/// The Errors that RRT may throw.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The version in the token string is not supported
    Version(VersionError),

    /// The network that was found is no part of the supported ones
    UnknownNetwork(u8),

    UnknownChannel(u8),

    /// The input string does not match the format (often the length) expected
    /// for the version. For instance, a V00 that is not 20 chars, or a V01 that is not 21 chars.
    /// The tuple is (expected, found) // TODO: use a srtuct to avoid having to document that
    LengthError(usize, usize),

    /// Some of the fields are encoded in hex. So those fields should be valid hexs strings.
    /// This error be thrown forinstance for a case_id that would not be a valid hex for instance.
    InvalidEncoding(String),

    /// The checksum is wrong.
    ChecksumError(ChecksumError),
}

impl From<VersionError> for Error {
    fn from(err: VersionError) -> Self {
        Self::Version(err)
    }
}
