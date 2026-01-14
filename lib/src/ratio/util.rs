//! Utility functions for keeping invariants of rational numbers.

#![expect(clippy::min_ident_chars, reason = "we are doing maths")]

use non_zero::non_zero;
use num::Integer;
use std::num::NonZeroU64;
use std::num::NonZeroU128;

/// Returns the greatest common divisor of two numbers.
pub(super) fn greatest_common_divisor(a: NonZeroU64, b: NonZeroU64) -> NonZeroU64 {
    NonZeroU64::new(Integer::gcd(&a.get(), &b.get())).unwrap_or(non_zero!(1))
}

/// Returns the greatest common divisor of two numbers.
pub(super) fn lowest_common_multiple(a: NonZeroU128, b: NonZeroU128) -> NonZeroU128 {
    NonZeroU128::new(Integer::lcm(&a.get(), &b.get())).unwrap_or(a.saturating_mul(b))
}
