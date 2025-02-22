use crate::ratio::Ratio;
use std::num::NonZeroU32;
use std::ops::{Div, DivAssign};

/// A non-zero `Ratio`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct NonZeroRatio {
    inner: Ratio,
}

impl NonZeroRatio {
    /// 1 / 4
    pub const QUARTER: NonZeroRatio = NonZeroRatio {
        inner: Ratio::QUARTER,
    };

    /// Creates a new `NonZeroRatio` representing `numerator` / `denominator`
    #[must_use]
    pub fn new(numerator: NonZeroU32, denominator: NonZeroU32) -> NonZeroRatio {
        // The non-zero types don't implement `num::Integer`.
        // Therefore, `num::rational::Ratio`s thereof cannot be reduced
        // since it requires comparison with zero.

        NonZeroRatio {
            inner: Ratio::new(numerator.get(), denominator.get()),
        }
    }

    /// Converts an integer to a `NonZeroRatio`
    #[must_use]
    pub fn int(integer: NonZeroU32) -> NonZeroRatio {
        NonZeroRatio {
            inner: Ratio::int(integer.get()),
        }
    }

    /// Converts `self` to a `Ratio`
    #[must_use]
    pub fn get(self) -> Ratio {
        self.inner
    }

    /// Converts a `Ratio` to a `NonZeroRatio` if it is not zero
    #[must_use]
    pub fn from_ratio(ratio: Ratio) -> Option<NonZeroRatio> {
        (ratio == Ratio::ZERO).then_some(NonZeroRatio { inner: ratio })
    }
}

impl Div for NonZeroRatio {
    type Output = NonZeroRatio;

    fn div(self, rhs: Self) -> Self::Output {
        #[expect(
            clippy::unwrap_used,
            reason = "numerator is non-zero, therefore the result will be too"
        )]
        NonZeroRatio::from_ratio(self.get() / rhs).unwrap()
    }
}

impl DivAssign for NonZeroRatio {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
