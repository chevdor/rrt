use crate::checksum::Checksum;
use crate::checksum::*;
use crate::utils::*;
use rand::Rng;
use std::convert::From;
use std::fmt::Display;
use std::str;

/// Supported channels
#[derive(Debug, PartialEq)]
pub enum Channel {
    Unknown,
    Email,
    Matrix,
    Twitter,
}

pub fn channel_to_string(c: &Channel) -> Option<String> {
    match c {
        Channel::Email => Some(String::from("EM")),
        Channel::Twitter => Some(String::from("TW")),
        Channel::Matrix => Some(String::from("MX")),
        _ => None,
    }
}

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

impl Display for RRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.format_string(&""))
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

/// Generate a random string of 'length' chars
/// The string is made of ascii chars from 65 to 90 (CAPS).
pub fn get_token(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<u8> = (0..length)
        .map(|_| {
            // 1 (inclusive) to 21 (exclusive)
            rng.gen_range(65..=90)
        })
        .collect();
    String::from(str::from_utf8(&chars).unwrap())
}

/// An RRT token looks like (dashes are for readability):
/// 01-00-02B21-TW-RAJQFIZW-O
/// Content is coded in hex.
/// 
/// You can display your RRT token using:
/// ```
/// use rrtlib::rrt::{Channel, RRT};
/// let token = RRT::new(1,0, 12345, Channel::Email);
/// println!("{}", token);
/// println!("{:?}", token);
/// println!("{:#?}", token);
/// println!("{}", token.format_string("-"));
/// ```
#[derive(Debug)]
pub struct RRT {
    /// Registrar index 0..255
    index: u8,

    /// RRT Token version 0..255
    version: u8,

    /// The case_id of our process
    case_id: u64,

    /// The channel
    channel: Channel,

    /// The random token
    token: String,

    /// The displayable checksum of the data
    checksum: u8,
}

impl RRT {
    /// Generate a new token and return a new RRT
    pub fn new(index: u8, version: u8, case_id: u64, channel: Channel) -> Self {
        let token = get_token(8);
        debug_assert!(
            token.len() == 8,
            "The generated token does not have the right length"
        );
        Self {
            index,
            version,
            case_id,
            channel,
            token,
            checksum: 0, // FIXME
        }
    }

    /// Unlike ::new(...), here you must pass the token
    pub fn new_with_token(
        index: u8,
        version: u8,
        case_id: u64,
        channel: Channel,
        token: &str,
    ) -> Self {
        assert!(
            token.len() == 8,
            "The passed token does not have the right length"
        );
        Self {
            index,
            version,
            case_id,
            channel,
            token: String::from(token),
            checksum: 0, // FIXME
        }
    }

    /// Allows formatting the token with separator. This is mainly used
    /// in the cli and for debugging.
    ///
    /// Example:
    /// ```
    /// use rrtlib::rrt::{Channel, RRT};
    /// let token = RRT::new(1, 0, 11041, Channel::Twitter);
    /// println!("{}", token.format_string("-"))
    /// ```
    pub fn format_string(&self, sep: &str) -> String {
        format!(
            "{RG}{S}{VV}{S}{CASE}{S}{CH}{S}{TOKEN_ID}{S}{C}",
            RG = dec2hex(self.index, 2),
            VV = dec2hex(self.version, 2),
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

        let s = &s.unwrap();
        let check = RRT::check(&s);
        if RRT::check(s).is_err() {
            panic!("Invalid RRT {}: {:?}", s, check.err());
            // return None;
        }

        // TODO: handle errors better
        let index = u8::from_str_radix(&s[0..2], 16).unwrap();
        let version = u8::from_str_radix(&s[2..4], 16).unwrap();
        let case_id = u64::from_str_radix(&s[4..9], 16).unwrap();
        let ch = &s[9..11];
        let channel = Channel::from(ch);
        let token = String::from(&s[11..19]);

        Ok(RRT::new_with_token(
            index, version, case_id, channel, &token,
        ))
    }

    pub fn clean_token_string(s: &str) -> Result<String, String> {
        const SIZE: usize = 20;

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
    pub fn check(s: &str) -> Result<(), String> {
        const SIZE: usize = 20;

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

        // If the token it too short, we cannot do much.. this is just wrong

        // println!("***** size ok");
        let raw = &cleaned[..SIZE - 1];
        // println!("***** raw: {}", raw);
        let expected = Checksum::calculate(raw);
        // println!("***** exp: {}", expected);
        let found: u8 = cleaned.as_bytes()[SIZE - 1];
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

    #[test]
    fn it_generate_a_8_chars_token() {
        let token = get_token(8);
        assert_eq!(token.len(), 8);
    }

    #[test]
    fn it_makes_a_rrt() {
        let token = RRT::new(1, 0, 11041, Channel::Twitter);
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_makes_a_rrt_with_token() {
        let token = RRT::new_with_token(1, 0, 11041, Channel::Twitter, "ABNCDEFG");
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_makes_a_token_from_string() {
        let token = RRT::from_string("010012345TWBABAEFGHJ").unwrap();
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_fails_making_a_token_from_bad_string() {
        let token = RRT::from_string("0100145TWBABAEFGHJ");
        assert!(token.is_err());
    }

    #[test]
    fn it_makes_a_token_from_string_with_seps() {
        let token = RRT::from_string("01-00_12345 TW/BABAEFGH:J").unwrap();
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_cleans_token_str() {
        assert_eq!(
            RRT::clean_token_string("010012345TWBABAEFGHJ").unwrap(),
            "010012345TWBABAEFGHJ"
        );
        assert_eq!(
            RRT::clean_token_string("01-00_12345 TW/BABAEFGH:J").unwrap(),
            "010012345TWBABAEFGHJ"
        );
        assert!(RRT::clean_token_string("010012345TWBABAEFGH").is_err());
    }

    #[test]
    fn it_generates_a_token() {
        let token = RRT::new(1, 0, 11041, Channel::Twitter);
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_generates_a_token_with() {
        let token = RRT::new_with_token(1, 0, 11041, Channel::Twitter, "12345678");
        assert_eq!(token.to_string().len(), 20);
    }

    #[test]
    fn it_passes_checksum_test() {
        let check = RRT::check("010002B21TWRAJQFIZWR");
        assert!(check.is_ok());
    }

    #[test]
    fn it_fails_with_bad_checksum() {
        assert!(RRT::check("JUNK").is_err());
        assert!(RRT::check("010002B21TWRAJQFIZWT").is_err());
    }

    #[test]
    fn it_parses_a_valid_token() {
        let token = "010002B21TWRAJQFIZWR";
        assert!(RRT::check(token).is_ok());
        assert_eq!(RRT::check(token), Ok(()));
    }

    #[test]
    fn it_parses_fields() {
        let t1 = RRT::new(1, 0, 12345, Channel::Twitter);
        assert_eq!(t1.index, 1);
        assert_eq!(t1.version, 0);
        assert_eq!(t1.case_id, 12345);
        assert_eq!(t1.channel, Channel::Twitter);
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
