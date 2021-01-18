use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Network {
    Polkadot = 0,
    Kusama = 2,
    Westend = 42,
    Unsupported,
}

// TODO: Unknown vs using Option<Channel> ?
impl From<&str> for Network {
    fn from(ch: &str) -> Self {
        return match &ch {
            &"00" => Network::Polkadot,
            &"02" => Network::Kusama,
            &"42" => Network::Westend,
            _ => Network::Unsupported,
        };
    }
}

impl Into<String> for Network {
    fn into(self) -> String {
        match self {
            Network::Polkadot => String::from("Polkadot"),
            Network::Kusama => String::from("Kusama"),
            Network::Westend => String::from("Westend"),
            _ => String::from("n/a"),
        }
    }
}

impl Display for Network {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}
