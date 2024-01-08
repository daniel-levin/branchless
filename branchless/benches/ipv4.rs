#![feature(test)]

extern crate branchless;
extern crate test;
use std::str::FromStr;

use test::Bencher;

static TEST_DATA: [&str; 10_000] = include!("../testdata/random");

#[bench]
fn bench_parse_inet4_single(bencher: &mut Bencher) {
    bencher.iter(|| {
        let _ = branchless::ip::parse_ipv4("127.0.0.1").unwrap();
    });
}

#[bench]
fn bench_parse_inet4_default_single(bencher: &mut Bencher) {
    bencher.iter(|| {
        let _ = std::net::Ipv4Addr::from_str("127.0.0.1").unwrap();
    });
}

#[bench]
fn bench_parse_inet4_sse2(bencher: &mut Bencher) {
    bencher.iter(|| {
        TEST_DATA.iter().for_each(|x| {
            let _ = branchless::ip::parse_ipv4(x);
        });
    });
}

#[bench]
fn bench_parse_inet4_default(bencher: &mut Bencher) {
    bencher.iter(|| {
        TEST_DATA.iter().for_each(|x| {
            let _ = std::net::Ipv4Addr::from_str(x);
        });
    });
}
