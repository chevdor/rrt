use crate::channel::channel_to_string;
pub use crate::channel::Channel;
use crate::checksum::*;
use crate::checksum_v00::ChecksumV00;
// use crate::checksum_v01::ChecksumV01;
pub use crate::network::Network;
use crate::utils::*;
pub use crate::version::Version;
use rand::Rng;
use std::convert::From;
use std::fmt::Display;
use std::str;

impl Display for RRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.format_string(&""))
    }
}

/// Generate a random string of 'length' chars
/// The string is made of ascii chars from 65 to 90 (CAPS).
pub fn get_token(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<u8> = (0..length).map(|_| rng.gen_range(65..=90)).collect();
    String::from(str::from_utf8(&chars).unwrap())
}

/// An RRT token looks like (dashes are for readability):
/// 01-00-02B21-TW-RAJQFIZW-O
/// Content is coded in hex.
///
/// You can display your RRT token using:
/// ```
/// use rrtlib::rrt::{Channel, Network, Version, RRT};
/// let token = RRT::new(Network::Polkadot, 1, Version::V00, 12345, Channel::Email);
/// println!("{}", token);
/// println!("{:?}", token);
/// println!("{:#?}", token);
/// println!("{}", token.format_string("-"));
/// ```
#[derive(Debug)]
pub struct RRT {
    /// Network
    network: Network,

    /// Registrar index 0..255
    index: u8,

    /// RRT Token version 0..255
    version: Version,

    /// The case_id of our process
    case_id: u64,

    /// The channel
    channel: Channel,

    /// The random token
    token: String,

    /// The supported checksum algos
    checksum_v00: ChecksumV00, // TODO: make it a Vec<>
                               // checksum_v01: ChecksumV01,
}

impl RRT {
    /// Generate a new token and return a new RRT
    pub fn new(
        network: Network,
        index: u8,
        version: Version,
        case_id: u64,
        channel: Channel,
    ) -> Self {
        let token = get_token(8);
        debug_assert!(
            token.len() == 8,
            "The generated token does not have the right length"
        );

        Self::new_with_token(
            network,
            index,
            version,
            case_id,
            channel,
            &String::from(token),
            None,
        )
    }

    /// Unlike ::new(...), here you must pass the token
    pub fn new_with_token(
        network: Network,
        index: u8,
        version: Version,
        case_id: u64,
        channel: Channel,
        token: &str,
        checksum: Option<u8>,
    ) -> Self {
        assert!(
            token.len() == 8,
            "The passed token does not have the right length"
        );
        Self {
            network,
            index,
            version,
            case_id,
            channel,
            token: String::from(token),
            checksum_v00: ChecksumV00::new(),
            // checksum_v01: ChecksumV01::new(),
        }
    }

    pub fn index(&self) -> u8 {
        self.index
    }

    pub fn version(&self) -> u8 {
        self.version as u8
    }

    pub fn network(&self) -> (u8, String) {
        (self.network as u8, "todo".into())
    }

    pub fn case_id(&self) -> u64 {
        self.case_id
    }

    pub fn channel(&self) -> Channel {
        self.channel
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn checksum(&self) -> Option<u8> {
        self.checksum_v00.checksum()
    }

    pub fn is_valid(&self) -> bool {
        self.checksum_v00.is_valid(&format!("{}", self).as_bytes())
    }

    // Returns the checksum algo to use depending on the version
    //TODO: we create new instances every time, can be improved
    // fn get_checksum_algo(version: Version) -> Box<dyn RRTChecksum> {
    //     match version {
    //         Version::V00 => Box::new(ChecksumV00::new()),
    //         Version::V01 => Box::new(ChecksumV00::new()),
    //     }
    // }

    /// Allows formatting the token with separator. This is mainly used
    /// in the cli and for debugging.
    ///
    /// Example:
    /// ```
    /// use rrtlib::rrt::{Channel, Network, Version, RRT};
    /// let token = RRT::new(Network::Polkadot, 1, Version::V00, 11041, Channel::Twitter);
    /// println!("{}", token.format_string("-"))
    /// ```
    pub fn format_string(&self, sep: &str) -> String {
        format!(
            "{RG}{S}{VV}{S}{NET}{S}{CASE}{S}{CH}{S}{TOKEN_ID}{S}{C}",
            RG = dec2hex(self.index, 2),
            VV = dec2hex(self.version as u8, 2),
            NET = dec2hex(self.network as u8, 2),
            CASE = dec2hex(self.case_id, 5),
            CH = channel_to_string(&self.channel).unwrap(), // FIXME
            TOKEN_ID = self.token,
            S = sep,
            C = "T"
        ) // FIXME
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let s = RRT::clean_token_string(s);
        if s.is_err() {
            return Err(format!(
                "The provided string could not be cleaned up: {:?}",
                s.err()
            ));
        }

        // Now we know that s is a token
        let s = &s.unwrap();

        // let v = Version::from(s);
        // let algo = RRT::get_checksum_algo(Version::V00);
        // let check = algo.calculate(s.as_bytes());
        let check = RRT::check(s, &ChecksumV00::new()); // TODO: stop making new ones...

        // if check.is_err() {
        //     panic!("Invalid RRT {}: {:?}", s, check.err());
        //     // return None;
        // }

        // 02_01_00_12345_TW_BABAEFGH_K (V00)
        // 0  2  4  6     11 13
        // TODO: handle errors better
        let network = Network::from(&s[0..2]);
        let index = u8::from_str_radix(&s[2..4], 16).unwrap();
        let version = Version::from(&s[4..6]);
        let case_id = u64::from_str_radix(&s[6..11], 16).unwrap();
        let channel = Channel::from(&s[11..13]);
        let token = String::from(&s[13..21]);
        let checksum = s.chars().nth(0).unwrap() as u8;
        Ok(RRT::new_with_token(
            network,
            index,
            version,
            case_id,
            channel,
            &token,
            Some(checksum),
        ))
    }

    /// This function
    pub fn clean_token_string(s: &str) -> Result<String, String> {
        const SIZE: usize = 22; // TODO: get that from the RRT struct the length depends also on the length of the checksum

        // TODO: workaround... https://github.com/rust-lang/rust/issues/37854
        const SIZEM: usize = SIZE - 1;

        match s.len() {
            0..=SIZEM => Err(format!(
                "Your string has a lenght of {}, this is too short. We expected >={}",
                s.len(),
                SIZE,
            )),
            SIZE => Ok(s.into()),
            _ => Ok(s
                .chars()
                // .map(|c| c as u8)
                .filter(|c| (*c >= 'A' && *c <= 'Z') || (*c >= '0' && *c <= '9'))
                .collect()),
        }
    }

    /// Returns whether a given token is valid or not.
    /// This function does that by re-caclulating the checksum and
    /// comparing with the one that was
    pub fn check(s: &str, algo: &ChecksumV00) -> Result<(), String> {
        const SIZE: usize = 22;

        // TODO: workaround... https://github.com/rust-lang/rust/issues/37854
        const SIZEM: usize = SIZE - 1;

        let cleaned: String = match s.len() {
            0..=SIZEM => format!("Invalid length. Got {}, expected {}", s.len(), SIZE,),

            SIZE => String::from(s),

            _ => match RRT::clean_token_string(&s) {
                Ok(s) => s,
                Err(e) => return Err(e),
            },
        };
        // println!("***** size ok");
        // println!("***** Working on {}", s);

        // let checksum = match self.version {
        //     Version::V00 => ChecksumV00::new(),
        //     // Version::V01 => ChecksumV01::new(),
        // };

        // If the token it too short, we cannot do much.. this is just wrong
        let raw = &cleaned[..SIZE - 1];
        let expected = algo.calculate(raw.as_bytes())[0];
        let found: u8 = cleaned.as_bytes()[SIZE - 1]; // TODO: RRTChecksum should have a get_checksum()
                                                      // println!("***** raw: {}", raw);
                                                      // println!("***** exp: {}", expected);
                                                      // println!("***** fnd: {}", found);
        match found == expected {
            true => Ok(()),
            false => Err(format!(
                "Wrong checksum in {}. Got {}={}, expected {}={}",
                s, found, found as char, expected, expected as char,
            )),
        }
    }
}

#[cfg(test)]
mod tests_rrt {
    use super::*;

    const CHAIN: Network = Network::Kusama;
    const VERSION: Version = Version::V00;

    #[test]
    fn it_generate_a_8_chars_token() {
        let token = get_token(8);
        assert_eq!(token.len(), 8);
    }

    #[test]
    fn it_makes_a_rrt() {
        let token = RRT::new(CHAIN, 1, VERSION, 11041, Channel::Twitter);
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_makes_a_rrt_with_token() {
        let token =
            RRT::new_with_token(CHAIN, 1, VERSION, 11041, Channel::Twitter, "ABNCDEFG", None);
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_makes_a_token_from_string() {
        let token = RRT::from_string("02010012345TWBABAEFQKQ").unwrap();
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_fails_making_a_token_from_bad_string() {
        let token = RRT::from_string("0100145TWBABAEFGHK");
        assert!(token.is_err());
    }

    #[test]
    fn it_makes_a_token_from_string_with_seps() {
        let token = RRT::from_string("42-01-00_12345 TW/BABAEFGH:H").unwrap();
        // orig: 42-01-00_12345 TW/BABAEFGH:J
        // mine:    42010012345TWBABAEFGHJ
        // cleaned: 42010012345TWBABAEFGHJ
        // raw:     42010012345TWBABAEF
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_cleans_token_str() {
        assert_eq!(
            RRT::clean_token_string("00010012345TWBABAEFGHJ").unwrap(),
            "00010012345TWBABAEFGHJ"
        );
        assert_eq!(
            RRT::clean_token_string("0001-00_12345 TW/BABAEFGH:J").unwrap(),
            "00010012345TWBABAEFGHJ"
        );

        // it fails when the string is too short from the start
        assert!(RRT::clean_token_string("42010012345TWB").is_err());
    }

    #[test]
    fn it_generates_a_token() {
        let token = RRT::new(CHAIN, 1, VERSION, 11041, Channel::Twitter);
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_generates_a_token_with() {
        let token =
            RRT::new_with_token(CHAIN, 1, VERSION, 11041, Channel::Twitter, "12345678", None);
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_passes_checksum_test() {
        let algo: ChecksumV00 = ChecksumV00::new();

        let check = RRT::check("01000012345TWRAJQFIZWF", &algo);
        // println!("check {:?}", check);
        assert!(check.is_ok());
    }

    #[test]
    fn it_fails_with_bad_checksum() {
        let algo: ChecksumV00 = ChecksumV00::new();

        assert!(RRT::check("JUNK", &algo).is_err());
        assert!(RRT::check("010002B21TWRAJQFIZWT", &algo).is_err());
    }

    #[test]
    fn it_parses_a_valid_token() {
        let algo: ChecksumV00 = ChecksumV00::new();

        let token = "01000012345TWRAJQFIZWF";
        // let check = RRT::check(&token);
        // println!("check {:?}", check);
        assert!(RRT::check(token, &algo).is_ok());
        assert_eq!(RRT::check(token, &algo), Ok(()));
    }

    #[test]
    fn it_parses_fields() {
        let t1 = RRT::new(CHAIN, 1, VERSION, 12345, Channel::Twitter);
        assert_eq!(t1.index, 1);
        assert_eq!(t1.version, Version::V00);
        assert_eq!(t1.case_id, 12345);
        assert_eq!(t1.channel, Channel::Twitter);
    }

    #[test]
    fn it_print_a_rrt_in_various_ways() {
        let rrt = RRT::new(CHAIN, 1, VERSION, 12345, Channel::Twitter);
        println!("{}", rrt);
        println!("{}", rrt.format_string("_"));
        println!("{:?}", rrt);
        println!("{:#?}", rrt);
    }
}

//   it('should parse and convert fields thru getters', () => {
//     [
//       [1, 0, 12345, Channel.Twitter],
//       [24, 255, 99999, Channel.Matrix],
//     ].forEach(
//       args => {
//         let [index, version, caseId, channel] = args;
//         let data = { index, version, caseId, channel }
//         let rrt_token = new RRT(index, version, caseId, channel).toString()
//         let rrt = RRT.fromString(rrt_token)

//         let fields = ['index', 'version', 'caseId', 'channel']
//         fields.forEach(f => {
//           expect(rrt[f]).equal(data[f])
//         })

//         expect(rrt.token.length).equal(8)
//         // console.log(rrt.toString('-'), rrt)
//         expect(rrt.checksum).above(41)
//       })
//   })
// })
