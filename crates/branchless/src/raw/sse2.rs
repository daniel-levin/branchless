#![cfg(target_arch = "x86_64")]
#![cfg(target_feature = "sse2")]

use core::arch::x86_64::{
    __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_set1_epi8,
};

type V = __m128i;

pub fn parse_ipv4(s: &str) -> Result<(), ()> {
    unsafe {
        // TODO: if the string is less than 128 bits long, we might read beyond its bounds.
        let v: V = _mm_loadu_si128(s.as_ptr() as *const V);

        let all_dots: V = _mm_set1_epi8(0x2E);
        let dot_locations: V = _mm_cmpeq_epi8(v, all_dots);
        let mask: i32 = _mm_movemask_epi8(dot_locations);

        Ok(())
    }
}
