#![expect(clippy::min_ident_chars, reason = "we are doing math")]

use num::Integer;
use std::num::{NonZeroU64, NonZeroU128};

const ONE: NonZeroU64 = NonZeroU64::MIN;

fn gcd(a: NonZeroU64, b: NonZeroU64) -> NonZeroU64 {
    NonZeroU64::new(Integer::gcd(&a.get(), &b.get())).unwrap_or(ONE)
}

pub(super) fn make_coprime(a: NonZeroU64, b: NonZeroU64) -> [NonZeroU64; 2] {
    let gcd = gcd(a, b);
    [
        NonZeroU64::new(a.get() / gcd).unwrap_or(ONE),
        NonZeroU64::new(b.get() / gcd).unwrap_or(ONE),
    ]
}

pub(super) fn lcm(a: NonZeroU128, b: NonZeroU128) -> NonZeroU128 {
    NonZeroU128::new(Integer::lcm(&a.get(), &b.get())).unwrap_or(a.saturating_mul(b))
}
