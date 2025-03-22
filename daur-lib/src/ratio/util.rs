#![expect(clippy::min_ident_chars, reason = "we are doing math")]

use num::Integer;
use std::num::{NonZeroU32, NonZeroU128};

const ONE: NonZeroU32 = NonZeroU32::MIN;

fn gcd(a: NonZeroU32, b: NonZeroU32) -> NonZeroU32 {
    NonZeroU32::new(Integer::gcd(&a.get(), &b.get())).unwrap_or(ONE)
}

pub(super) fn make_coprime(a: NonZeroU32, b: NonZeroU32) -> [NonZeroU32; 2] {
    let gcd = gcd(a, b);
    [
        NonZeroU32::new(a.get() / gcd).unwrap_or(ONE),
        NonZeroU32::new(b.get() / gcd).unwrap_or(ONE),
    ]
}

pub(super) fn lcm(a: NonZeroU128, b: NonZeroU128) -> NonZeroU128 {
    NonZeroU128::new(Integer::lcm(&a.get(), &b.get())).unwrap_or(a.saturating_mul(b))
}
