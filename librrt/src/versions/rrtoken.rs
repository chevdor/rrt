use crate::types::Version;
use crate::utils::dec2hex;
use crate::versions::*;
use crate::Channel;
use crate::Network;
use enum_dispatch::enum_dispatch;
use std::fmt::Display;

#[enum_dispatch]
#[derive(Debug)]
pub enum Token {
    V00(TokenV00),
    V01(TokenV01),
}

impl Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self)
    }
}

// Ignore the following wiggles, this is RLS bug: https://gitlab.com/antonok/enum_dispatch/-/issues/21
#[enum_dispatch(Token)]
pub trait Tokenize: std::fmt::Debug + std::fmt::Display {
    /// Returns the size (=length) of the tokens managed by a RRT token.
    fn size_of(&self) -> usize;

    /// Returns true if both the version and the length of the candidate string
    /// match the implementation.
    // fn is_candidate(&self, s: &str) -> bool;

    fn app(&self) -> &u8;
    fn version(&self) -> &Version;

    fn network(&self) -> &Network;
    fn index(&self) -> &u8;
    fn channel(&self) -> &Channel;
    fn case_id(&self) -> &u64;
    fn secret(&self) -> &String;

    fn checksum(&self) -> String;

    /// Allows formatting the token with separator. This is mainly used
    /// in the cli and for debugging.
    ///
    /// Example:
    /// // TODO: make the following a compilied example again once archtecture is stable
    /// use rrtlib::types::{Channel, Network, Version, RRT};
    /// use rttlib::versions::token_v00::TokenV00;
    /// let token = TokenV00::new(Network::Polkadot, 1, Version::V00, 11041, Channel::Twitter);
    /// println!("{}", token.format_string("-"))
    fn format_string(&self, sep: &str) -> String {
        format!(
            // 01 00 02 01 02B21 TW 12345678 75
            "{APP}{S}{VV}{S}{NET}{S}{RG}{S}{CASE}{S}{CH}{S}{_SECRET_}{S}{C}",
            APP = dec2hex(*self.app() as u8, 2),
            VV = dec2hex(*self.version() as u8, 2),
            RG = dec2hex(*self.index(), 2),
            NET = dec2hex(*self.network() as u8, 2),
            CASE = dec2hex(*self.case_id(), 5),
            CH = &self.channel().to_string(),
            _SECRET_ = self.secret(),
            S = sep,
            C = self.checksum().to_string()
        )
    }
}

#[macro_export]
macro_rules! gen_getter {
    ($name: ident, $type: ty) => {
        fn $name(&self) -> $type {
            &self.$name
        }
    };
}
