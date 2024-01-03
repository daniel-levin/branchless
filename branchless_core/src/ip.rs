//! IP address parsing.

#[cfg(target_arch = "x86_64")]
#[cfg(target_feature = "sse2")]
pub use crate::raw::sse2::parse_ipv4;

#[derive(Debug, PartialEq, Eq)]
pub enum Ipv4ParseError {
    WrongLength,
    Invalid,
}
