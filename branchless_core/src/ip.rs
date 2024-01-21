//! IP address parsing.

#[cfg(target_arch = "x86_64")]
#[cfg(target_feature = "sse2")]
pub use crate::raw::sse2::parse_ipv4;

/// Coarse-grained error for [parse_ipv4].
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Ipv4ParseError {
    /// Expected a string with length between 7 and 15.
    WrongLength,

    /// Otherwise invalid input.
    Invalid,
}
