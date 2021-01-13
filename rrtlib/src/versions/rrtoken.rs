use crate::types::version::Version;
use crate::versions::token_v00::TokenV00;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
#[derive(Debug)]
pub enum Token {
    V00(TokenV00),
}

// Ignore the following wiggles, this is RLS bug: https://gitlab.com/antonok/enum_dispatch/-/issues/21
#[enum_dispatch(Token)]
pub trait Tokenize: std::fmt::Debug {
    /// Returns the size (=length) of the tokens managed by a RRT token.
    fn size_of(&self) -> usize;

    /// Returns true if both the version and the length of the candidate string
    /// match the implementation.
    fn is_candidate(&self, s: &str) -> bool;

    fn version(&self) -> Version;
}

// pub trait TokenBaseImpl {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//         write!(f, "{}", self.format_string(&""))
//     }

// }
