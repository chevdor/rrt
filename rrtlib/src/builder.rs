//! This builder does:
//! - take an input string
//! - tries to figure out an RRT Version that can be used to build a RRT struct
//! - does the job

use crate::detector::Detector;
use crate::types::channel::*;
use crate::types::network::*;
use crate::types::version::*;
use crate::versions::rrtoken::Token;
use crate::versions::rrtoken::Tokenize;
use crate::versions::token_v00::TokenV00;

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
        let size = some_tuple.1;
        let version = match some_tuple.0 {
            None => panic!("Fix me, got no version 2"),
            Some(v) => v,
        };

        match version {
            Version::V00 => Some(TokenV00::new(
                version,
                Network::Kusama,
                1,
                11041,
                Channel::Twitter,
            )),
            _ => todo!(),
        }
    }

    /// This function return a given token
    pub fn build_with_variant(s: &str) -> Option<Token> {
        let analysis = Detector::analyze(s);
        let some_tuple = analysis.expect("Fix me, got no version 1");
        let size = some_tuple.1;
        let version = match some_tuple.0 {
            None => panic!("Fix me, got no version 2"),
            Some(v) => v,
        };

        match version {
            Version::V00 => Some(Token::V00(TokenV00::new(
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
    use crate::error::error::Error;

    #[test]
    fn it_returns_a_tokenize() {
        let s = "00000012345TWRAJQFIZWF";
        let analysis = Detector::analyze(s);
        assert_eq!(Ok((Some(Version::V00), 22)), analysis);

        let tkn = Builder::build(s).expect("Got None where we expected Some Token_V00");
        println!("RESULT: {:?}", tkn);

        // TODO: I need to get the variant back... but do I ?
        println!(
            "We lost the variant but we know this is version {:?}",
            tkn.version()
        );
    }

    #[test]
    fn it_returns_a_variant() {
        let s = "00000012345TWRAJQFIZWF";
        let analysis = Detector::analyze(s);
        assert_eq!(Ok((Some(Version::V00), 22)), analysis);

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
        let s = "01000012345TWRAJQFIZWFX";
        let analysis = Detector::analyze(s);
        assert_eq!(Ok((Some(Version::V01), 23)), analysis);
    }

    #[test]
    fn it_runs_3() {
        let s = "02000012345TWRAJQFIZWFX";
        let analysis = Detector::analyze(s);
        assert_eq!(
            Err(Error::Version(VersionError::UnsupportedVersion(2))),
            analysis
        );
    }
}
