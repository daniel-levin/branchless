use std::str::FromStr;

static TEST_DATA: [&'static str; 10_000] = include!("../testdata/random");

#[test]
fn test_parity_parse_ipv4() {
    for ip in TEST_DATA {
        let sse2 = branchless::ip::parse_ipv4(ip).unwrap();
        let def = std::net::Ipv4Addr::from_str(ip).unwrap();

        assert_eq!(sse2, def);
    }
}
