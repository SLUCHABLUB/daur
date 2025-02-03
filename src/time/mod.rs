use num::{rational, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, ToPrimitive};
use saturating_cast::{SaturatingCast, SaturatingElement};
use std::fmt::{Display, Formatter};
use std::num::NonZeroU8;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

pub mod bar;
pub mod duration;
pub mod instant;
pub mod period;
pub mod signature;
pub mod tempo;

/// A ratio between two `u64`s.
/// When operations would result in a non-representable value, the result is an approximation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Ratio {
    inner: rational::Ratio<u64>,
}

impl Ratio {
    pub const ZERO: Ratio = Ratio {
        inner: rational::Ratio::ZERO,
    };

    pub fn new(numerator: u64, denominator: u64) -> Ratio {
        Ratio {
            inner: rational::Ratio::new(numerator, denominator),
        }
    }

    pub const fn new_raw(numerator: u64, denominator: u64) -> Ratio {
        Ratio {
            inner: rational::Ratio::new_raw(numerator, denominator),
        }
    }

    pub fn ceil(self) -> u64 {
        self.inner.ceil().to_integer()
    }

    pub fn to_float(self) -> f64 {
        #![allow(clippy::cast_precision_loss)]
        self.inner
            .to_f64()
            .unwrap_or_else(|| *self.inner.numer() as f64 / *self.inner.denom() as f64)
    }

    pub fn approximate(float: f64) -> Ratio {
        Ratio {
            inner: rational::Ratio::<u64>::from_f64(float).unwrap_or_default(),
        }
    }

    // TODO: in operations, we don't need to reduce both operands
    fn force_reduced(self) -> Ratio {
        let (mut numerator, mut denominator) = self.inner.into_raw();
        numerator /= 2;
        denominator /= 2;

        if denominator == 0 {
            denominator = 1;
        }

        let inner = rational::Ratio::new_raw(numerator, denominator);
        Ratio { inner }
    }

    fn force_reduced_non_zero(self) -> Ratio {
        let (mut numerator, mut denominator) = self.inner.into_raw();
        numerator /= 2;
        denominator /= 2;

        if numerator == 0 {
            numerator = 1;
        }
        if denominator == 0 {
            denominator = 1;
        }

        let inner = rational::Ratio::new_raw(numerator, denominator);
        Ratio { inner }
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<NonZeroU8> for Ratio {
    fn from(value: NonZeroU8) -> Self {
        Ratio {
            inner: rational::Ratio::from(u64::from(value.get())),
        }
    }
}

impl From<u16> for Ratio {
    fn from(value: u16) -> Self {
        Ratio {
            inner: rational::Ratio::from(u64::from(value)),
        }
    }
}

impl Add<Ratio> for Ratio {
    type Output = Ratio;

    fn add(self, rhs: Ratio) -> Ratio {
        if let Some(inner) = self.inner.checked_add(&rhs.inner) {
            Ratio { inner }
        } else {
            self.force_reduced() + rhs.force_reduced()
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
            self.force_reduced() - rhs.force_reduced()
        }
    }
}

impl Mul for Ratio {
    type Output = Ratio;

    fn mul(self, rhs: Ratio) -> Ratio {
        if let Some(inner) = self.inner.checked_mul(&rhs.inner) {
            Ratio { inner }
        } else {
            self.force_reduced() * rhs.force_reduced()
        }
    }
}

impl MulAssign for Ratio {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div<Ratio> for Ratio {
    type Output = Ratio;

    fn div(self, rhs: Ratio) -> Ratio {
        debug_assert_ne!(rhs, Ratio::ZERO, "tried dividing by zero");
        if let Some(inner) = self.inner.checked_div(&rhs.inner) {
            Ratio { inner }
        } else {
            self.force_reduced() / rhs.force_reduced_non_zero()
        }
    }
}

impl SaturatingElement<u64> for Ratio {
    fn as_element(self) -> u64 {
        self.inner.round().to_integer()
    }
}

impl SaturatingElement<i32> for Ratio {
    fn as_element(self) -> i32 {
        self.saturating_cast::<u64>().saturating_cast()
    }
}

impl SaturatingCast for Ratio {}
