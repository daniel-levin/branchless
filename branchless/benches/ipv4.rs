#![feature(test)]

extern crate branchless;
extern crate test;
use std::str::FromStr;

use test::Bencher;

static TEST_DATA: [&'static str; 10_000] = include!("../testdata/random");

#[bench]
fn bench_parse_inet4_sse2(bencher: &mut Bencher) {
    bencher.iter(|| {
        TEST_DATA.map(|x| branchless::ip::parse_ipv4(x).unwrap());
    });
}

#[bench]
fn bench_parse_inet4_default(bencher: &mut Bencher) {
    bencher.iter(|| {
        TEST_DATA.map(|x| std::net::Ipv4Addr::from_str(x).unwrap());
    });
}
