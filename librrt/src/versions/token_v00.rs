use crate::*;

// #![feature(trace_macros)]

use crate::error::Error;
use crate::utils::utils::dec2hex;
use rand::Rng;
use std::convert::From;
use std::fmt::Display;
use std::str;
use std::str::FromStr;

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
///
#[derive(Debug)]
pub struct TokenV00 {
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

    checksum: Option<u8>,
}

impl Display for TokenV00 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.format_string(&""))
    }
}

// TODO: move to default impl in the trait
/// Generate a random string of 'length' chars
/// The string is made of ascii chars from 65 to 90 (CAPS).
pub fn get_token(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<u8> = (0..length).map(|_| rng.gen_range(65..=90)).collect();
    String::from(str::from_utf8(&chars).unwrap())
}

impl Tokenize for TokenV00 {
    fn size_of(&self) -> usize {
        let token = TokenV00::new(0, Version::V00, Network::Kusama, 1, 1234, Channel::Email);
        token.format_string("").len()
    }

    gen_getter!(app, &u8);
    gen_getter!(version, &Version);

    gen_getter!(network, &Network);
    gen_getter!(channel, &Channel);
    gen_getter!(index, &u8);
    gen_getter!(case_id, &u64);
    gen_getter!(secret, &String);

    fn checksum(&self) -> String {
        let chk = match self.checksum {
            None => String::from("?"),
            Some(x) => format!("{}", x),
        };

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
        let token = &get_token(8);
        debug_assert!(
            token.len() == 8,
            "The generated token does not have the right length"
        );

        Self::new_with_token(app, version, network, index, case_id, channel, token, None)
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
        checksum: Option<u8>,
    ) -> Self {
        assert!(
            secret.len() == 8,
            "The passed secret does not have the right length"
        );

        let chk = match checksum {
            Some(c) => Some(c),
            None => {
                let mut algo = ChecksumV00::new();
                let raw =
                    TokenV00::format_raw(app, version, network, index, case_id, channel, secret);
                Some(algo.calculate(raw.as_bytes()))
            }
        };

        Self {
            app,
            version,
            network,
            index,
            case_id,
            channel,
            secret: String::from(secret),
            checksum: chk,
        }
    }

    // TODO: remove this, it was a test
    pub fn special(&self) -> String {
        String::from("I am a special string only V00 can return")
    }

    pub fn is_valid(&self) -> bool {
        false
        // TODO: dont forget that
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
            "{APP}{VV}{RG}{NET}{CASE}{CH}{_SECRET_}{C}",
            APP = dec2hex(app as u8, 2),
            VV = dec2hex(version as u8, 2),
            RG = dec2hex(index, 2),
            NET = dec2hex(network as u8, 2),
            CASE = dec2hex(case_id, 5),
            CH = channel_to_string(&channel).unwrap(), // FIXME, we should Impl. Display instead
            _SECRET_ = secret,
            C = "T"
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
    ///
    pub fn format_string(&self, sep: &str) -> String {
        format!(
            "{APP}{S}{VV}{S}{RG}{S}{NET}{S}{CASE}{S}{CH}{S}{_SECRET_}{S}{C}",
            APP = dec2hex(self.app as u8, 2),
            VV = dec2hex(self.version as u8, 2),
            RG = dec2hex(self.index, 2),
            NET = dec2hex(self.network as u8, 2),
            CASE = dec2hex(self.case_id, 5),
            CH = channel_to_string(&self.channel).unwrap(), // FIXME, we should Impl. Display instead
            _SECRET_ = self.secret,
            S = sep,
            C = "T"
        ) // FIXME
    }

    // TODO: move to the root trait with def. impl
    /// This function removes any char that is not part of [A-Z0-9]
    pub fn clean_token_string(s: &str) -> String {
        s.chars()
            .filter(|c| (*c >= 'A' && *c <= 'Z') || (*c >= '0' && *c <= '9'))
            .collect()
    }

    /// Returns whether a given token is valid or not.
    /// This function does that by re-caclulating the checksum and
    /// comparing with the one that was
    pub fn check(s: &str, algo: &mut dyn Checksum<u8>) -> Result<(), String> {
        const SIZE: usize = 24;

        // TODO: workaround... https://github.com/rust-lang/rust/issues/37854
        const SIZEM: usize = SIZE - 1;

        let cleaned: String = match s.len() {
            0..=SIZEM => format!("Invalid length. Got {}, expected {}", s.len(), SIZE),
            SIZE => String::from(s),
            _ => TokenV00::clean_token_string(&s),
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
        let s = TokenV00::clean_token_string(s);

        if s.len() < 24 {
            return Err(Error::LengthError(24, s.len()));
        }

        // 01_02_01_00_12345_TW_BABAEFGH_K (V00)
        // 0  2  4  6  8     13 15
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
        let token = String::from(&s[15..23]);
        let checksum = s
            .chars()
            .nth(0)
            .expect(&format!("Bad token {}, length={}", s, s.len())) as u8;
        Ok(TokenV00::new_with_token(
            app,
            version,
            network,
            index,
            case_id,
            channel,
            &token,
            Some(checksum),
        ))
    }
}

#[cfg(test)]
mod tests_rrt {
    use super::*;

    const APP: u8 = 0;
    const CHAIN: Network = Network::Kusama;
    const VERSION: Version = Version::V00;

    #[test]
    fn it_generate_a_8_chars_token() {
        let token = get_token(8);
        assert_eq!(token.len(), 8);
    }

    #[test]
    fn it_makes_a_rrt() {
        let token = TokenV00::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        assert_eq!(token.size_of(), token.to_string().len());
    }

    #[test]
    fn it_makes_a_rrt_with_correct_checksum() {
        let token = TokenV00::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        let mut chk = ChecksumV00::new();
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
        assert_eq!(24, token.size_of());
    }

    #[test]
    fn it_makes_a_rrt_with_token() {
        let token = TokenV00::new_with_token(
            APP,
            VERSION,
            CHAIN,
            1,
            11041,
            Channel::Twitter,
            "ABNCDEFG",
            None,
        );
        assert_eq!(token.to_string().len(), token.size_of());
    }

    #[test]
    fn it_makes_a_token_from_string() {
        let token = TokenV00::from_str("0000010012345TWBABAEFQKQ").unwrap();
        assert_eq!(token.to_string().len(), token.size_of());
    }

    #[test]
    fn it_fails_making_a_token_from_bad_string() {
        let token = TokenV00::from_str("0100145TWBABAEFGHK");
        assert!(token.is_err());
    }

    #[test]
    fn it_makes_a_token_from_string_with_seps() {
        let token = TokenV00::from_str("11-00-42-00_12345 TW/BABAEFGH:H").unwrap();
        assert_eq!(token.to_string().len(), token.size_of());
    }

    #[test]
    fn it_cleans_token_str() {
        let samples = [
            ("00010012345TWBABAEFGHJ", "00010012345TWBABAEFGHJ"),
            ("0001-00_12345 TW/BABAEFGH:J", "00010012345TWBABAEFGHJ"),
            ("42010012345TWB", "42010012345TWB"),
        ];

        for s in &samples {
            assert_eq!(TokenV00::clean_token_string(s.0), s.1)
        }
    }

    #[test]
    fn it_generates_a_token() {
        let token = TokenV00::new(APP, VERSION, CHAIN, 1, 11041, Channel::Twitter);
        assert_eq!(token.to_string().len(), token.size_of());
    }

    #[test]
    fn it_generates_a_token_with() {
        let token = TokenV00::new_with_token(
            APP,
            VERSION,
            CHAIN,
            1,
            11041,
            Channel::Twitter,
            "12345678",
            None,
        );
        assert_eq!(token.to_string().len(), token.size_of());
    }

    #[test]
    fn it_passes_checksum_test() {
        let mut algo: ChecksumV00 = ChecksumV00::new();
        let check = TokenV00::check("0001000012345TWRAJQFIZWX", &mut algo);
        assert!(check.is_ok());
    }

    #[test]
    fn it_fails_with_bad_checksum() {
        let mut algo: ChecksumV00 = ChecksumV00::new();

        assert!(TokenV00::check("JUNK", &mut algo).is_err());
        assert!(TokenV00::check("010002B21TWRAJQFIZWT", &mut algo).is_err());
    }

    #[test]
    fn it_parses_a_valid_token() {
        let mut algo: ChecksumV00 = ChecksumV00::new();
        let token = "1101000012345TWRAJQFIZWZ";
        let res = TokenV00::check(token, &mut algo);
        assert!(res.is_ok());
        assert_eq!(TokenV00::check(token, &mut algo), Ok(()));
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
