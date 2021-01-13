use crate::error::error::Error::*;
use crate::error::error::*;
use crate::types::version::Version;
use std::str::FromStr;

/// The detector is not doing much parsing beside the version and the length.
/// It helps finding the appropriate builders that will parse and check the full
/// String.
#[derive(Debug)]
pub struct Detector {}

pub struct MatchResult {
    version: Option<Version>,
    length: usize,
}

impl Detector {
    /// Takes a String and return a Tuple made of
    /// the detected version and the size of the string
    pub fn analyze(s: &str) -> Result<(Option<Version>, usize), Error> {
        match s.len() {
            x if x < 2 => Err(LengthError(2, s.len())),
            _ => {
                let version_str = &s[0..2];
                let version = Version::from_str(version_str)?;
                // TODO: handle the case when from_str returns an error
                return Ok((Some(version), s.len()));
            }
        }
    }
}

#[cfg(test)]
mod tests_detector {
    use super::*;
    use crate::types::version::VersionError::*;

    #[test]
    fn it_always_return() {
        assert!(Detector::analyze("A").is_err());

        assert_eq!(
            Ok((Some(Version::V00), 22)),
            Detector::analyze("00000012345TWRAJQFIZWF")
        );
        assert_eq!(
            Ok((Some(Version::V01), 22)),
            Detector::analyze("01000012345TWRAJQFIZWF")
        );
        assert_eq!(
            Err(Error::Version(UnsupportedVersion(2))),
            Detector::analyze("02000012345TWRAJQFIZWF")
        );
    }
}
