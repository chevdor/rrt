use crate::error::Error;
use crate::utils::*;
use crate::*;
use std::convert::From;
use std::fmt::Display;
use std::str;
use std::str::FromStr;

const TOKEN_V00_SIZE: usize = 24;

/// An RRT token looks like (dashes are for readability):
/// 01-00-02B21-TW-RAJQFIZW-O
/// Content is coded in hex.
///
/// You can display your RRT token using:
///
/// // TODO: make compilable again once the arch is stable
/// use rrtlib::types::{Channel, Network, Version, RRT};
/// use rttlib::versions:token_v00::TokenV00;
/// let token = TokenV00::new(Network::Polkadot, 1, Version::V00, 12345, Channel::Email);
/// println!("{}", token);
/// println!("{:?}", token);
/// println!("{:#?}", token);
/// println!("{}", token.format_string("-"));
/// 01 00 02 01 02B21 TW 12345678 T
#[derive(Debug)]
pub struct TokenV00 {
    /// App
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

    /// The random secret token
    secret: String,

    checksum: ChecksumOutput,
}

impl Display for TokenV00 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.format_string(&""))
    }
}

impl Tokenize for TokenV00 {
    fn size_of(&self) -> usize {
        TOKEN_V00_SIZE
    }

    gen_getter!(app, &u8);
    gen_getter!(version, &Version);
    gen_getter!(network, &Network);
    gen_getter!(channel, &Channel);
    gen_getter!(index, &u8);
    gen_getter!(case_id, &u64);
    gen_getter!(secret, &String);

    fn checksum(&self) -> String {
        let chk = &self.checksum.to_string();
        format!("{}", chk)
    }
}

impl TokenV00 {
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
        Self::new_with_token(app, version, network, index, case_id, channel, token)
    }

    /// Unlike ::new(...), here you must pass the token
    pub fn new_with_token(
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

        let algo = ChecksumV00::new();
        let raw = TokenV00::format_raw(app, version, network, index, case_id, channel, secret);
        let checksum = ChecksumOutput::Single(algo.calculate(raw.as_bytes()));

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

    // TODO: remove this, it was a test
    pub fn special(&self) -> String {
        String::from("I am a special string only V00 can return")
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
            CH = channel_to_string(&channel).unwrap(), // FIXME, we should Impl. Display instead
            _SECRET_ = secret,
        )
    }

    /// Allows formatting the token with separator. This is mainly used
    /// in the cli and for debugging.
    ///
    /// Example:
    /// // TODO: make the following a compilied example again once archtecture is stable
    /// use rrtlib::types::{Channel, Network, Version, RRT};
    /// use rttlib::versions::token_v00::TokenV00;
    /// let token = TokenV00::new(Network::Polkadot, 1, Version::V00, 11041, Channel::Twitter);
    /// println!("{}", token.format_string("-"))
    pub fn format_string(&self, sep: &str) -> String {
        format!(
            // 01 00 02 01 02B21 TW 12345678 75
            "{APP}{S}{VV}{S}{NET}{S}{RG}{S}{CASE}{S}{CH}{S}{_SECRET_}{S}{C}",
            APP = dec2hex(self.app as u8, 2),
            VV = dec2hex(self.version as u8, 2),
            RG = dec2hex(self.index, 2),
            NET = dec2hex(self.network as u8, 2),
            CASE = dec2hex(self.case_id, 5),
            CH = channel_to_string(&self.channel).unwrap(), // FIXME, we should Impl. Display instead
            _SECRET_ = self.secret,
            S = sep,
            C = self.checksum().to_string()
        )
    }

    /// Returns whether a given token is valid or not.
    /// This function does that by re-caclulating the checksum and
    /// comparing with the one that was
    pub fn check(s: &str, algo: &dyn Checksum<u8>) -> Result<(), String> {
        const SIZE: usize = TOKEN_V00_SIZE;

        // TODO: workaround... https://github.com/rust-lang/rust/issues/37854
        const SIZEM: usize = SIZE - 1;

        let cleaned: String = match s.len() {
            0..=SIZEM => format!("Invalid length. Got {}, expected {}", s.len(), SIZE),
            SIZE => String::from(s),
            _ => clean_token_string(&s),
        };

        // If the string it too short, we cannot do much.. this is just wrong
        let raw = &cleaned[..SIZE - 1];
        let expected = algo.calculate(raw.as_bytes());
        let found: u8 = cleaned.as_bytes()[SIZE - 1];
        println!("***** raw: {}", raw);
        println!("***** exp: {}/{}", expected, expected as char);
        println!("***** fnd: {}/{}", found, found as char);
        match found == expected {
            true => Ok(()),
            false => Err(format!(
                "Wrong checksum in {}. Got {}={}, expected {}={}",
                s, found, found as char, expected, expected as char,
            )),
        }
    }
}

impl FromStr for TokenV00 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = clean_token_string(s);
        if s.len() < TOKEN_V00_SIZE {
            return Err(Error::LengthError(TOKEN_V00_SIZE, s.len()));
        }

        // 01_02_01_00_12345_TW_BABAEFGH_K (V00)
        // 0  2  4  6  8     13 15       24
        // TODO: handle errors better
        let app = u8::from_str_radix(&s[0..2], 16).expect(&format!(
            "Bad token {}, length={}",
            s,
            s.len()
        ));
        let version = Version::from_str(&s[2..4])?;
        let network = Network::from(&s[4..6]);
        let index = u8::from_str_radix(&s[6..8], 16).expect(&format!(
            "Bad token {}, length={}",
            s,
            s.len()
        ));
        let case_id = u64::from_str_radix(&s[8..13], 16).expect(&format!(
            "Bad token {}, length={}",
            s,
            s.len()
        ));
        let channel = Channel::from(&s[13..15]);
        let secret = String::from(&s[15..23]);
        let checksum_str =
            s.chars()
                .nth_back(0)
                .expect(&format!("Bad token {}, length={}", s, s.len())) as u8;

        let algo = ChecksumV00::new();
        let raw = TokenV00::format_raw(app, version, network, index, case_id, channel, &secret);
        let checksum = ChecksumOutput::Single(algo.calculate(raw.as_bytes()));

        match checksum {
            ChecksumOutput::Single(s) if s == checksum_str => Ok(Self {
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
                ChecksumOutput::Single(checksum_str),
            ))),
        }
    }
}

#[cfg(test)]
mod tests_rrt {
    use super::*;

    const APP: u8 = 0;
    const CHAIN: Network = Network::Kusama;
    const VERSION: Version = Version::V00;

    #[test]
    fn it_makes_a_rrt() {
        let token = TokenV00::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        assert_eq!(TOKEN_V00_SIZE, token.to_string().len());
    }

    #[test]
    fn it_makes_a_rrt_with_correct_checksum() {
        let token = TokenV00::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        let chk = ChecksumV00::new();
        let c1 = chk.calculate(token.to_string().as_bytes());
        let c2 = ChecksumV00::from_str(&token.to_string())
            .unwrap()
            .checksum()
            .unwrap();
        assert_eq!(c1, c2);
    }

    #[test]
    fn it_returns_the_correct_size() {
        let token = TokenV00::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        assert_eq!(TOKEN_V00_SIZE, token.size_of());
    }

    #[test]
    fn it_makes_a_rrt_with_token() {
        let token =
            TokenV00::new_with_token(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter, "ABNCDEFG");
        assert_eq!(TOKEN_V00_SIZE, token.to_string().len());
    }

    #[test]
    fn it_makes_a_token_from_string() {
        let token = TokenV00::from_str("0000010012345TWBABAEFQKD");
        assert!(token.is_ok());
        assert_eq!(TOKEN_V00_SIZE, token.unwrap().to_string().len());
    }

    #[test]
    fn it_fails_making_a_token_from_bad_string() {
        let token = TokenV00::from_str("0100145TWBABAEFGHK");
        assert!(token.is_err());
    }

    #[test]
    fn it_makes_a_token_from_string_with_seps() {
        let token = TokenV00::from_str("11-00-02-00_12345 TW/BABAEFGH:A");
        assert!(token.is_ok());
        assert_eq!(TOKEN_V00_SIZE, token.unwrap().to_string().len());
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
        let token = TokenV00::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        assert_eq!(TOKEN_V00_SIZE, token.to_string().len());
    }

    #[test]
    fn it_generates_a_token_with() {
        let t = TokenV00::new_with_token(1, VERSION, CHAIN, 1, 11041, Channel::Twitter, "12345678");
        assert_eq!("0100020102B21TW12345678K", t.to_string());
        assert_eq!(TOKEN_V00_SIZE, t.to_string().len());
    }

    #[test]
    fn it_passes_checksum_test() {
        let algo: ChecksumV00 = ChecksumV00::new();
        let check = TokenV00::check("0001000012345TWRAJQFIZWX", &algo);
        assert!(check.is_ok());
    }

    #[test]
    fn it_fails_with_bad_checksum() {
        let algo: ChecksumV00 = ChecksumV00::new();

        assert!(TokenV00::check("JUNK", &algo).is_err());
        assert!(TokenV00::check("010002B21TWRAJQFIZWT", &algo).is_err());
    }

    #[test]
    fn it_parses_a_valid_token() {
        let algo: ChecksumV00 = ChecksumV00::new();
        let token = "1101000012345TWRAJQFIZWZ";
        let res = TokenV00::check(token, &algo);
        assert!(res.is_ok());
        assert_eq!(TokenV00::check(token, &algo), Ok(()));
    }

    #[test]
    fn it_parses_fields() {
        let t1 = TokenV00::new(APP, VERSION, CHAIN, 1, 12345, Channel::Twitter);
        assert_eq!(t1.index, 1);
        assert_eq!(t1.version, Version::V00);
        assert_eq!(t1.case_id, 12345);
        assert_eq!(t1.channel, Channel::Twitter);
    }

    #[test]
    fn it_print_a_rrt_in_various_ways() {
        let rrt = TokenV00::new(APP, VERSION, CHAIN, 1, 12345, Channel::Twitter);
        println!("{}", rrt);
        println!("{}", rrt.format_string("_"));
        println!("{:?}", rrt);
        println!("{:#?}", rrt);
    }
}
