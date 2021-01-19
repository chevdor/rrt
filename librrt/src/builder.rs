//! This builder does:
//! - take an input string
//! - tries to figure out an RRT Version that can be used to build a RRT struct
//! - does the job

use crate::detector::Detector;
use crate::types::*;
use crate::versions::*;
use std::str::FromStr;

pub struct Builder {
    // tokens: Vec<Token>,
}

impl Builder {
    pub fn new() -> Self {
        // Self { tokens: Vec::new() }
        Self {}
    }

    //TODO return a Result instead
    /// This function return 'a' token implementing Tokenize but we lost which one
    pub fn build(s: &str) -> Option<impl Tokenize + std::fmt::Debug> {
        let analysis = Detector::analyze(s);
        let some_tuple = analysis.expect("Fix me, got no version 1");

        let app = match some_tuple.0 {
            None => panic!("Fix me, got no version 2"),
            Some(v) => v,
        };

        let version = match some_tuple.1 {
            None => panic!("Fix me, got no version 2"),
            Some(v) => v,
        };

        let size = some_tuple.2;
        match (app, version, size) {
            (_x, Version::V00, 24) => Some(Token::from(
                TokenV00::from_str(s).expect("Invalid token v00"),
            )),
            (_x, Version::V01, 25) => Some(Token::from(
                TokenV01::from_str(s).expect("Invalid token v01"),
            )),
            (x, v, l) => panic!(format!(
                "Wooooo we don't support that: App={app} v{version} with length= {length}",
                length = l,
                version = v,
                app = x,
            )),
        }
    }

    /// This function return a given token
    pub fn build_with_variant(s: &str) -> Option<Token> {
        let analysis = Detector::analyze(s);
        let some_tuple = analysis.expect("Fix me, got no version 1");

        let app = match some_tuple.0 {
            None => panic!("Fix me, got no version 2"),
            Some(v) => v,
        };

        let version = match some_tuple.1 {
            None => panic!("Fix me, got no version 2"),
            Some(v) => v,
        };

        let size = some_tuple.2;
        match (app, version, size) {
            (x, Version::V00, 24) => Some(Token::V00(TokenV00::new(
                x,
                version,
                Network::Kusama,
                1,
                11041,
                Channel::Twitter,
            ))),
            (x, Version::V01, 25) => Some(Token::V01(TokenV01::new(
                x,
                version,
                Network::Kusama,
                1,
                11041,
                Channel::Twitter,
            ))),
            _ => todo!(),
        }
    }

    // pub fn add_token(&mut self, token: Token) {
    //     self.tokens.push(token);
    // }

    // pub fn get_handler() -> Result<RRTToken, Error> {
    // pub fn get_handler(s: &str) -> () {
    //     ()
    // }
}

#[cfg(test)]
mod tests_builder {
    use super::*;
    use crate::detector::Detector;
    use crate::error::Error;

    #[test]
    fn it_returns_a_tokenize() {
        let s = "0000000012345TWRAJQFIZWW";
        let analysis = Detector::analyze(s);
        assert_eq!(Ok((Some(0), Some(Version::V00), 24)), analysis);
        let tkn = Builder::build(s).expect("Got None where we expected Some Token_V00");
        println!(
            "We lost the variant but we know this is version {:?}",
            tkn.version()
        );
    }

    #[test]
    fn it_returns_a_variant() {
        let s = "0000000012345TWRAJQFIZWW";
        let analysis = Detector::analyze(s);
        assert_eq!(Ok((Some(0), Some(Version::V00), 24)), analysis);

        let tkn_variant =
            Builder::build_with_variant(s).expect("Got None where we expected Some Token_V00");
        println!("RESULT: {:?}", tkn_variant);

        match tkn_variant {
            Token::V00(t) => println!("Special: {}", t.special()),
            _ => println!("Some other version"),
        };
    }

    #[test]
    #[ignore]
    fn it_runs_2() {
        let s = "0301000012345TWRAJQFIZWFX";
        let analysis = Detector::analyze(s);
        assert_eq!(Ok((Some(3), Some(Version::V01), 25)), analysis);
    }

    #[test]
    fn it_runs_3() {
        let s = "FF02000012345TWRAJQFIZWFX";
        let analysis = Detector::analyze(s);
        assert_eq!(
            Err(Error::Version(VersionError::UnsupportedVersion(2))),
            analysis
        );
    }
}
