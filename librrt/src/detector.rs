use crate::error::*;
use crate::types::Version;
use crate::Error::LengthError;
use std::str::FromStr;

/// The detector is not doing much parsing beside the version and the length.
/// It helps finding the appropriate builders that will parse and check the full
/// String.
#[derive(Debug)]
pub struct Detector {}

impl Detector {
    /// Takes a String and return a Tuple made of
    /// the detected version and the size of the string
    pub fn analyze(s: &str) -> Result<(Option<u8>, Option<Version>, usize), Error> {
        match s.len() {
            x if x < 2 => Err(LengthError(2, s.len())),
            _ => {
                let app = u8::from_str_radix(&s[0..2], 16).unwrap();
                let version_str = &s[2..4];
                let version = Version::from_str(version_str)?;
                return Ok((Some(app), Some(version), s.len()));
            }
        }
    }
}

#[cfg(test)]
mod tests_detector {
    use super::*;
    use crate::types::VersionError::*;

    #[test]
    fn it_always_return() {
        let samples = [
            (
                "0000000012345TWRAJQFIZWF",
                (Some(0), Some(Version::V00), 24),
            ),
            (
                "0A01000012345TWRAJQFIZWF",
                (Some(10), Some(Version::V01), 24),
            ),
        ];

        for s in &samples {
            assert_eq!(Ok(s.1), Detector::analyze(s.0));
        }
    }

    #[test]
    fn it_catches_errors() {
        assert!(Detector::analyze("A").is_err());

        assert_eq!(
            Err(Error::Version(UnsupportedVersion(2))),
            Detector::analyze("0202000012345TWRAJQFIZWF")
        );
    }
}
