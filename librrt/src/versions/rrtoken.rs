use crate::types::Version;
use crate::versions::*;
use crate::Channel;
use crate::Error;
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
}

#[macro_export]
macro_rules! gen_getter {
    ($name: ident, $type: ty) => {
        fn $name(&self) -> $type {
            &self.$name
        }
    };
}
