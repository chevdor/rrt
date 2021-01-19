//! This file contains the list of the supported versions
use std::convert::Into;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Version {
    /// This is the initial implementation version. This is a rather naive but
    /// simple and fast implementation. It is however not robust aginst position swapping.
    V00 = 0x0,

    /// This version aims at improving the robustness against position swapping using the Fletcher 16 algorithm.
    V01 = 0x01,
    // This version does not exist yet. It will likely be reserved for a Blake implementation test.
    // V02 = 0x02,
    // ...
    //
    //VFF = 0xFF,
}

#[derive(Debug, PartialEq)]
pub enum VersionError {
    /// The version in the token string is not supported
    ParseError(String),

    UnsupportedVersion(u8),
}

impl FromStr for Version {
    type Err = VersionError;
    fn from_str(vstr: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        const START: usize = 4;
        let v: &str = match vstr.len() {
            2 => &vstr,
            x if x < 2 => panic!(format!("Cannot get a version in {}", vstr)),
            _ => &vstr[START..START + 2],
        };

        return match &v {
            &"00" => Ok(Version::V00),
            &"01" => Ok(Version::V01),
            // &"02" => Version::V02,
            v if v.parse::<u8>().is_ok() => {
                Err(VersionError::UnsupportedVersion(v.parse().unwrap()))
            }
            _ => Err(VersionError::ParseError(String::from(vstr))),
        };
    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        format!("{:02?}", self)
    }
}

impl LowerHex for Version {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:02?}", *self as u32)
    }
}

impl Display for Version {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:02?}", *self as u32)
    }
}

#[cfg(test)]
mod tests_rrt {
    use super::*;

    #[test]
    fn it_converts_to_string() {
        let v1 = Version::V01;
        assert_eq!("01", &v1.to_string());
    }

    #[test]
    fn it_converts_from_string() {
        assert_eq!(Version::from_str("01"), Ok(Version::V01));
    }

    #[test]
    fn it_errors_on_unparsable_version() {
        assert_eq!(
            Version::from_str("XX"),
            Err(VersionError::ParseError("XX".into()))
        );
    }

    #[test]
    fn it_errors_on_unsupported_version() {
        assert_eq!(
            Version::from_str("99"),
            Err(VersionError::UnsupportedVersion(99))
        );
    }

    #[test]
    fn it_converts_from_bad_string2() {
        let _ = Version::from_str("01010212345TWBABAEFGH");
    }

    #[test]
    fn it_converts_from_full_token() {
        assert_eq!(Version::from_str("01000012345TWBABAEFGH"), Ok(Version::V00));
        assert_eq!(Version::from_str("01010112345TWBABAEFGH"), Ok(Version::V01));
    }
}
