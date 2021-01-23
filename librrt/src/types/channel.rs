use std::fmt::Display;

/// Supported channels
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Channel {
    Unknown,
    Email,
    Matrix,
    Twitter,
}

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

impl Channel {
    pub fn format_str(&self) -> String {
        let str = match self {
            Channel::Email => "Email",
            Channel::Twitter => "Twitter",
            Channel::Matrix => "Matrix",
            _ => "n/a",
        };

        String::from(str)
    }
}

impl Display for Channel {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let str = match self {
            Channel::Email => "EM",
            Channel::Twitter => "TW",
            Channel::Matrix => "MX",
            _ => "XX",
        };

        write!(fmt, "{}", String::from(str))
    }
}

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
