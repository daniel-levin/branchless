#![cfg(target_arch = "x86_64")]
#![cfg(target_feature = "sse2")]

use branchless::ip::parse_ipv4;

#[test]
fn test_parse() {
    let localhost = parse_ipv4("127.0.0.1").unwrap();
}
