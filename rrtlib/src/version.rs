//! This file contains the list of the supported versions
use std::convert::Into;
use std::fmt::Display;

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

impl From<&str> for Version {
    fn from(vstr: &str) -> Self {
        const START: usize = 4;
        let v: &str = match vstr.len() {
            2 => &vstr[START..START + 2],
            _ => &vstr,
        };

        return match &v {
            &"00" => Version::V00,
            &"01" => Version::V01,
            // &"02" => Version::V02,
            _ => panic!(format!("Version {:?} is currently not supported", v)),
        };
    }
}

// TODO: make is generic for all types implementing Trait Debug
impl Into<String> for Version {
    fn into(self) -> String {
        format!("{:02?}", self)
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
        assert_eq!(v1.to_string(), String::from("01"));
    }

    #[test]
    fn it_converts_from_string() {
        assert_eq!(Version::from("01"), Version::V01);
    }

    #[test]
    #[should_panic(expected = "not supported")]
    fn it_converts_from_bad_string() {
        let _ = Version::from("XX");
    }

    #[test]
    #[should_panic(expected = "not supported")]
    fn it_converts_from_bad_string2() {
        let _ = Version::from("01010212345TWBABAEFGH");
    }

    #[test]
    fn it_converts_from_full_token() {
        assert_eq!(Version::from("01000012345TWBABAEFGH"), Version::V00);
        assert_eq!(Version::from("01010112345TWBABAEFGH"), Version::V01);
    }
}
