pub mod bar;
pub mod duration;
pub mod instant;
pub mod period;
mod signature;
pub mod tempo;

pub use signature::TimeSignature;
use std::fmt;

use num::{
    rational, CheckedAdd as _, CheckedDiv as _, CheckedMul as _, CheckedSub as _,
    FromPrimitive as _, ToPrimitive as _,
};
use std::fmt::{Display, Formatter};
use std::num::NonZeroU8;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

// TODO: non-zero version
/// A rational number.
/// When operations would result in a non-representable value, the result is an approximation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Ratio {
    inner: rational::Ratio<u32>,
}

impl Ratio {
    pub const ZERO: Ratio = Ratio {
        inner: rational::Ratio::ZERO,
    };

    pub const QUARTER: Ratio = Ratio {
        inner: rational::Ratio::new_raw(1, 4),
    };

    pub fn new(numerator: u32, denominator: u32) -> Self {
        Ratio {
            inner: rational::Ratio::new(numerator, denominator),
        }
    }

    pub fn ceil(self) -> u32 {
        self.ceiled().inner.to_integer()
    }

    pub fn ceiled(self) -> Ratio {
        Ratio {
            inner: self.inner.ceil(),
        }
    }

    pub fn round(self) -> u32 {
        self.rounded().inner.to_integer()
    }

    pub fn rounded(self) -> Ratio {
        Ratio {
            inner: self.inner.round(),
        }
    }
}

impl Ratio {
    pub fn approximate(float: f64) -> Ratio {
        Ratio {
            inner: rational::Ratio::from_f64(float).unwrap_or_default(),
        }
    }

    pub fn to_float(self) -> f64 {
        self.inner
            .to_f64()
            .unwrap_or_else(|| f64::from(*self.inner.numer()) / f64::from(*self.inner.denom()))
    }
}

impl Ratio {
    fn approximate_big(big: rational::Ratio<u128>) -> Ratio {
        if big == rational::Ratio::ZERO {
            return Ratio::ZERO;
        }

        let (numerator, denominator) = big.into_raw();
        Ratio::approximate_u128(numerator, denominator)
    }

    fn approximate_u128(mut numerator: u128, mut denominator: u128) -> Ratio {
        if let Ok(numerator) = u32::try_from(numerator) {
            if let Ok(denominator) = u32::try_from(denominator) {
                return Ratio {
                    inner: rational::Ratio::new_raw(numerator, denominator),
                };
            }
        }

        numerator = u128::max(numerator >> 1, 1);
        denominator = u128::max(denominator >> 1, 1);

        Ratio::approximate_u128(numerator, denominator)
    }

    /// Due to using lcm (multiplication) in addition to addition in addition (in reduction),
    /// we need to use u128 as opposed to u64 for the result
    fn big_inner(self) -> rational::Ratio<u128> {
        let (numerator, denominator) = self.inner.into_raw();
        rational::Ratio::new_raw(u128::from(numerator), u128::from(denominator))
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<NonZeroU8> for Ratio {
    fn from(value: NonZeroU8) -> Self {
        Ratio {
            #[expect(
                clippy::non_zero_suggestions,
                reason = "we don't care about non-zeroness"
            )]
            inner: rational::Ratio::from(u32::from(value.get())),
        }
    }
}

impl Add<Ratio> for Ratio {
    type Output = Ratio;

    fn add(self, rhs: Ratio) -> Ratio {
        if let Some(inner) = self.inner.checked_add(&rhs.inner) {
            Ratio { inner }
        } else {
            #[expect(clippy::arithmetic_side_effects, reason = "see `Ratio::big_inner`")]
            Ratio::approximate_big(self.big_inner() + rhs.big_inner())
        }
    }
}

impl AddAssign<Ratio> for Ratio {
    fn add_assign(&mut self, rhs: Ratio) {
        *self = *self + rhs;
    }
}

impl Sub for Ratio {
    type Output = Ratio;

    fn sub(self, rhs: Self) -> Self::Output {
        if self <= rhs {
            return Ratio::ZERO;
        }

        if let Some(inner) = self.inner.checked_sub(&rhs.inner) {
            Ratio { inner }
        } else {
            #[expect(clippy::arithmetic_side_effects, reason = "see `Ratio::big_inner`")]
            Ratio::approximate_big(self.big_inner() - rhs.big_inner())
        }
    }
}

impl Mul for Ratio {
    type Output = Ratio;

    fn mul(self, rhs: Ratio) -> Ratio {
        if let Some(inner) = self.inner.checked_mul(&rhs.inner) {
            Ratio { inner }
        } else {
            #[expect(clippy::arithmetic_side_effects, reason = "see `Ratio::big_inner`")]
            Ratio::approximate_big(self.big_inner() * rhs.big_inner())
        }
    }
}

impl MulAssign for Ratio {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// TODO: non-zero type
impl Div<Ratio> for Ratio {
    type Output = Ratio;

    fn div(self, rhs: Ratio) -> Ratio {
        debug_assert_ne!(rhs, Ratio::ZERO, "tried dividing by zero");
        if let Some(inner) = self.inner.checked_div(&rhs.inner) {
            Ratio { inner }
        } else {
            #[expect(clippy::arithmetic_side_effects, reason = "see `Ratio::big_inner`")]
            Ratio::approximate_big(self.big_inner() / rhs.big_inner())
        }
    }
}
