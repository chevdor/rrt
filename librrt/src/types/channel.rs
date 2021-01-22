use std::fmt::Display;

/// Supported channels
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Channel {
    Unknown,
    Email,
    Matrix,
    Twitter,
}

// TODO: Use a more Rusty way...
// pub fn channel_to_string(c: &Channel) -> Option<String> {
//     match c {
//         Channel::Email => Some(String::from("EM")),
//         Channel::Twitter => Some(String::from("TW")),
//         Channel::Matrix => Some(String::from("MX")),
//         _ => None,
//     }
// }

// TODO: what is better: Unknown vs using Option<Channel> ?
impl From<&str> for Channel {
    fn from(ch: &str) -> Self {
        return match &ch {
            &"TW" => Channel::Twitter,
            &"EM" => Channel::Email,
            &"MX" => Channel::Matrix,
            _ => Channel::Unknown,
        };
    }
}

impl From<String> for Channel {
    fn from(ch: String) -> Self {
        Channel::from(ch)
    }
}

impl Display for Channel {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let str = match self {
            Channel::Email => "EM",
            Channel::Twitter => "TW",
            Channel::Matrix => "MX",
            _ => "n/a",
        };

        write!(fmt, "{}", String::from(str))
    }
}

// pub fn string_to_channel(s: &str) -> Option<Channel> {
//     match s.to_ascii_uppercase().as_str() {
//         "EM" => Some(Channel::Email),
//         "TW" => Some(Channel::Twitter),
//         "MX" => Some(Channel::Matrix),
//         _ => None,
//     }
// }

#[cfg(test)]
mod tests_rrt {
    use super::*;

    #[test]
    fn it_converts_to_string() {
        assert_eq!(&Channel::Email.to_string(), "EM");
    }

    #[test]
    fn it_converts_from_string() {
        assert_eq!(Channel::from("EM"), Channel::Email);
    }

    #[test]
    fn it_converts_from_bad_string() {
        assert_eq!(Channel::from("XX"), Channel::Unknown);
    }
}
