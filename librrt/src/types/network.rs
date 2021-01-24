use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Network {
    Known(KnownNetwork),
    Unknown(u8),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KnownNetwork {
    Polkadot = 0,
    Kusama = 2,
    Westend = 42,
}

impl From<&str> for Network {
    fn from(n: &str) -> Self {
        let n = u8::from_str_radix(n, 16).expect(&format!("Failed parsing {} as u8", n));
        Network::from(n)
    }
}

impl From<u8> for Network {
    fn from(n: u8) -> Self {
        return match n {
            0 => Network::Known(KnownNetwork::Polkadot),
            2 => Network::Known(KnownNetwork::Kusama),
            42 => Network::Known(KnownNetwork::Westend),
            x => Network::Unknown(x),
        };
    }
}

impl Into<String> for Network {
    fn into(self) -> String {
        match self {
            Network::Known(KnownNetwork::Polkadot) => String::from("Polkadot"),
            Network::Known(KnownNetwork::Kusama) => String::from("Kusama"),
            Network::Known(KnownNetwork::Westend) => String::from("Westend"),
            Network::Unknown(n) => format!("Network {:02?}", n),
        }
    }
}

impl Into<u8> for Network {
    fn into(self) -> u8 {
        match self {
            Network::Known(KnownNetwork::Polkadot) => 0,
            Network::Known(KnownNetwork::Kusama) => 2,
            Network::Known(KnownNetwork::Westend) => 42,
            Network::Unknown(n) => n,
        }
    }
}

impl Display for Network {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}
