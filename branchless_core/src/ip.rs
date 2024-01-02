//! IP address parsing.

#[cfg(target_arch = "x86_64")]
#[cfg(target_feature = "sse2")]
pub use crate::raw::sse2::parse_ipv4;

#[derive(Debug)]
pub enum Ipv4ParseError {
    TooLong,
    Invalid,
}
