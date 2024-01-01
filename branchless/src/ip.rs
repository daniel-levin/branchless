#![cfg(target_arch = "x86_64")]
#![cfg(target_feature = "sse2")]
pub fn parse_ipv4(s: &str) -> Result<std::net::Ipv4Addr, ()> {
    Ok(std::net::Ipv4Addr::from(u32::to_le_bytes(
        branchless_core::ip::parse_ipv4(s)?,
    )))
}
