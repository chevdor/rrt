#[derive(Debug, PartialEq)]
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
