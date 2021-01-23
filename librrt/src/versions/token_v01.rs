use crate::checksum::*;
use crate::error::Error;
use crate::utils::*;
use crate::versions::rrtoken::Tokenize;
use crate::*;
use std::convert::From;
use std::fmt::{Debug, Display};
use std::str;
use std::str::FromStr;

const TOKEN_V01_SIZE: usize = 25;

/// An RRT token looks like (dashes are for readability):
/// 01-00-02B21-TW-RAJQFIZW-O
/// Content is coded in hex.
///
/// You can display your RRT token using:
/// ```
/// use librrt::*;
/// let token = TokenV01::new(0, Version::V00, Network::Polkadot, 1, 12345, Channel::Email);
/// println!("{}", token);
/// println!("{:?}", token);
/// println!("{:#?}", token);
/// println!("{}", token.format_string("-"));
/// ```
#[derive(Debug)]
pub struct TokenV01 {
    /// A numerical index representing the App
    app: u8,

    /// RRT Token version 0..255
    version: Version,

    /// Network
    network: Network,

    /// Registrar index 0..255
    index: u8,

    /// The case_id of our process
    case_id: u64,

    /// The channel
    channel: Channel,

    /// The random token
    secret: String,

    checksum: ChecksumOutput,
}

impl Display for TokenV01 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.format_string(&""))
    }
}

impl FromStr for TokenV01 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = clean_token_string(s);
        if s.len() < TOKEN_V01_SIZE {
            return Err(Error::LengthError(TOKEN_V01_SIZE, s.len()));
        }

        // 01_02_01_00_12345_TW_BABAEFGH_KX (V01)
        // 0  2  4  6  8     13 15       24
        // TODO: handle errors better
        let app = u8::from_str_radix(&s[0..2], 16).unwrap();
        let version = Version::from_str(&s[2..4])?;
        let network_index = u8::from_str_radix(&s[4..6], 16).unwrap();
        let network = Network::from(network_index);
        let index = u8::from_str_radix(&s[6..8], 16).unwrap();
        let case_id = u64::from_str_radix(&s[8..13], 16).unwrap();
        let channel = Channel::from(&s[13..15]);
        let secret = String::from(&s[15..23]);
        let checksum_str = Self::extract_checksum(&s)?;

        let algo = ChecksumV01::new();
        let raw = TokenV01::format_raw(app, version, network, index, case_id, channel, &secret);
        let checksum = ChecksumOutput::Dual(algo.calculate(raw.as_bytes()));

        match checksum {
            ChecksumOutput::Dual(d) if d == checksum_str => Ok(Self {
                app,
                version,
                network,
                index,
                case_id,
                channel,
                secret,
                checksum,
            }),
            _ => Err(Error::ChecksumError(ChecksumError::new(
                s,
                checksum,
                ChecksumOutput::Dual(checksum_str),
            ))),
        }
    }
}

impl Tokenize for TokenV01 {
    fn size_of(&self) -> usize {
        TOKEN_V01_SIZE
    }

    gen_getter!(app, &u8);
    gen_getter!(version, &Version);
    gen_getter!(network, &Network);
    gen_getter!(channel, &Channel);
    gen_getter!(index, &u8);
    gen_getter!(case_id, &u64);
    gen_getter!(secret, &String);

    fn checksum(&self) -> String {
        let chk = match &self.checksum {
            ChecksumOutput::Single(s) => panic!(format!(
                "Did not expect a 'Single' Checksum in V01, got {:?}",
                s
            )),
            ChecksumOutput::Dual(d) => format!("{}", String::from_utf8_lossy(d)),
        };

        format!("{}", chk)
    }
}

impl TokenV01 {
    /// Generate a new token and return a new RRT
    pub fn new(
        app: u8,
        version: Version,
        network: Network,
        index: u8,
        case_id: u64,
        channel: Channel,
    ) -> Self {
        let token = &gen_random_string(8);
        Self::new_with_secret(
            app,
            version,
            network,
            index,
            case_id,
            channel,
            &String::from(token),
        )
    }

    fn format_raw(
        app: u8,
        version: Version,
        network: Network,
        index: u8,
        case_id: u64,
        channel: Channel,
        secret: &str,
    ) -> String {
        format!(
            "{APP}{VV}{RG}{NET}{CASE}{CH}{_SECRET_}",
            APP = dec2hex(app as u8, 2),
            VV = dec2hex(version as u8, 2),
            RG = dec2hex(index, 2),
            NET = dec2hex(network as u8, 2),
            CASE = dec2hex(case_id, 5),
            CH = &channel.to_string(),
            _SECRET_ = secret,
        )
    }

    /// Unlike ::new(...), here you must pass the secret
    pub fn new_with_secret(
        app: u8,
        version: Version,
        network: Network,
        index: u8,
        case_id: u64,
        channel: Channel,
        secret: &str,
    ) -> Self {
        assert!(
            secret.len() == 8,
            "The passed secret does not have the right length"
        );

        let algo = ChecksumV01::new();
        let raw = TokenV01::format_raw(app, version, network, index, case_id, channel, secret);
        let checksum = ChecksumOutput::Dual(algo.calculate(raw.as_bytes()));

        Self {
            app,
            version,
            network,
            index,
            case_id,
            channel,
            secret: String::from(secret),
            checksum,
        }
    }

    fn extract_checksum(s: &str) -> Result<[u8; 2], Error> {
        // TODO: can do better...
        if s.len() < TOKEN_V01_SIZE {
            return Err(Error::LengthError(25, s.len()));
        }
        let a = s.as_bytes()[TOKEN_V01_SIZE - 2];
        let b = s.as_bytes()[TOKEN_V01_SIZE - 1];
        Ok([a, b])
    }

    /// Returns whether a given token is valid or not.
    /// This function does that by re-caclulating the checksum and
    /// comparing with the one that was
    pub fn check(s: &str, algo: &dyn Checksum<[u8; 2]>) -> Result<(), Error> {
        const SIZE: usize = TOKEN_V01_SIZE;

        // TODO: workaround... https://github.com/rust-lang/rust/issues/37854
        const SIZEM: usize = SIZE - 2;

        let cleaned: String = match s.len() {
            0..=SIZEM => format!("Invalid length. Got {}, expected {}", s.len(), SIZE),
            SIZE => String::from(s),
            _ => clean_token_string(&s),
        };

        // If the string it too short, we cannot do much.. this is just wrong
        let raw = &cleaned[..SIZE - 2];
        let expected = algo.calculate(raw.as_bytes());
        let found = TokenV01::extract_checksum(s)?;

        // println!("***** raw: {}", raw);
        // println!("***** exp: {:?}", expected);
        // println!("***** fnd: {:?}", found);
        match found == expected {
            true => Ok(()),
            false => Err(Error::ChecksumError(ChecksumError::new(
                s.into(),
                ChecksumOutput::Dual(expected),
                ChecksumOutput::Dual(found),
            ))),
        }
    }
}

#[cfg(test)]
mod tests_rrt {
    use super::*;

    const CHAIN: Network = Network::Westend;
    const VERSION: Version = Version::V01;
    const APP: u8 = 0;

    #[test]
    fn it_makes_a_rrt() {
        let token = TokenV01::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        assert_eq!(TOKEN_V01_SIZE, token.to_string().len());
    }

    #[test]
    fn it_returns_the_correct_size() {
        let token = TokenV01::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        assert_eq!(TOKEN_V01_SIZE, token.size_of());
    }

    #[test]
    fn it_makes_a_rrt_with_token() {
        let token =
            TokenV01::new_with_secret(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter, "ABNCDEFG");
        println!("{}", token);
        assert_eq!(TOKEN_V01_SIZE, token.to_string().len());
    }

    #[test]
    fn it_makes_a_token_from_string() {
        let token = TokenV01::from_str("1100010012345TWBABAEFQKJA");
        assert!(token.is_ok());
        assert_eq!(TOKEN_V01_SIZE, token.unwrap().to_string().len());
    }

    #[test]
    fn it_fails_making_a_token_from_bad_string() {
        let token = TokenV01::from_str("0100145TWBABAEFGHK");
        assert!(token.is_err());
    }

    #[test]
    fn it_makes_a_token_from_string_with_seps() {
        let token = TokenV01::from_str("11-01-32-00_12345 TW/BABAEFGH:GA");
        assert!(token.is_ok());
        assert_eq!(TOKEN_V01_SIZE, token.unwrap().to_string().len());
    }

    #[test]
    fn it_cleans_token_str() {
        let samples = [
            ("00010012345TWBABAEFGHJ", "00010012345TWBABAEFGHJ"),
            ("0001-00_12345 TW/BABAEFGH:J", "00010012345TWBABAEFGHJ"),
            ("42010012345TWB", "42010012345TWB"),
        ];

        for s in &samples {
            assert_eq!(clean_token_string(s.0), s.1)
        }
    }

    #[test]
    fn it_generates_a_token() {
        let token = TokenV01::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        assert_eq!(TOKEN_V01_SIZE, token.to_string().len());
    }

    #[test]
    fn it_generates_a_token_with() {
        let token =
            TokenV01::new_with_secret(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter, "12345678");
        assert_eq!(TOKEN_V01_SIZE, token.to_string().len());
    }

    #[test]
    fn it_passes_checksum_test() {
        let algo: ChecksumV01 = ChecksumV01::new();
        let check = TokenV01::check("1101000012345TWRAJQFIZWGG", &algo);
        assert!(check.is_ok());
    }

    #[test]
    fn it_fails_with_bad_checksum() {
        let algo: ChecksumV01 = ChecksumV01::new();

        assert!(TokenV01::check("JUNK", &algo).is_err());
        assert!(TokenV01::check("010002B21TWRAJQFIZWT", &algo).is_err());
    }

    #[test]
    fn it_parses_a_valid_token() {
        let algo: ChecksumV01 = ChecksumV01::new();
        let token = "1101000012346TWRAJQFIZWRP";
        let res = TokenV01::check(token, &algo);
        println!("res= {:?}", res);
        assert!(res.is_ok());
        assert_eq!(TokenV01::check(token, &algo), Ok(()));
    }

    #[test]
    fn it_parses_fields() {
        let t1 = TokenV01::new(APP, VERSION, CHAIN, 1, 12345, Channel::Twitter);
        assert_eq!(t1.index, 1);
        assert_eq!(t1.version, Version::V01);
        assert_eq!(t1.case_id, 12345);
        assert_eq!(t1.channel, Channel::Twitter);
    }

    #[test]
    fn it_print_a_rrt_in_various_ways() {
        let rrt = TokenV01::new(APP, VERSION, CHAIN, 1, 12345, Channel::Twitter);
        println!("{}", rrt);
        println!("{}", rrt.format_string("_"));
        println!("{:?}", rrt);
        println!("{:#?}", rrt);
    }

    /// Works for strings with separators
    /// 00_01_2A_01_03039_TW_JXBACTSP_RR
    /// returns
    /// 2A
    fn get_network(s: &str) -> String {
        let network = &s[6..8];
        String::from(network)
    }

    #[test]
    fn it_generates_a_token_with_valid_network_westend() {
        let token = TokenV01::from_str("00_01_2A_01_03039_TW_JXBACTSP_RR").unwrap();
        let network = get_network(&token.format_string("_"));
        assert_eq!(token.network, Network::Westend);
        assert_eq!(network, "2A"); // 42 in hex = 2A
    }

    #[test]
    fn it_generates_a_token_with_valid_network_kusama() {
        let token = TokenV01::from_str("00_01_02_01_03039_TW_JXBACTSP_YY").unwrap();
        let network = get_network(&token.format_string("_"));
        assert_eq!(token.network, Network::Kusama);
        assert_eq!(network, "02");
    }

    #[test]
    fn it_generates_a_token_with_valid_checksum() {
        let s = "0001020103039TWJXBACTSPYY";
        let token = TokenV01::from_str(s).unwrap();
        let checksum_str = TokenV01::extract_checksum(&s).unwrap();
        assert_eq!([89, 89], checksum_str);
        assert_eq!(String::from_utf8_lossy(&[89, 89]), token.checksum());
    }
}
