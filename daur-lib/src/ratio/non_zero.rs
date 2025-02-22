use crate::ratio::util::make_coprime;
use crate::ratio::{Ratio, FOUR, ONE};
use std::cmp::Ordering;
use std::num::{NonZeroU128, NonZeroU32};
use std::ops::{Div, DivAssign};

/// A non-zero `Ratio`
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NonZeroRatio {
    numerator: NonZeroU32,
    denominator: NonZeroU32,
}

impl NonZeroRatio {
    /// 1 / 4
    pub const QUARTER: NonZeroRatio = NonZeroRatio::reciprocal_of(FOUR);

    /// 1
    pub const ONE: NonZeroRatio = NonZeroRatio::integer(ONE);

    const MIN: NonZeroRatio = NonZeroRatio::reciprocal_of(NonZeroU32::MAX);
    const MAX: NonZeroRatio = NonZeroRatio::integer(NonZeroU32::MAX);

    /// Creates a new `NonZeroRatio` representing `numerator` / `denominator`
    #[must_use]
    pub fn new(numerator: NonZeroU32, denominator: NonZeroU32) -> NonZeroRatio {
        // The non-zero types don't implement `num::Integer`.
        // Therefore, `num::rational::Ratio`s thereof cannot be reduced
        // since it requires comparison with zero.

        let [numerator, denominator] = make_coprime(numerator, denominator);

        NonZeroRatio {
            numerator,
            denominator,
        }
    }

    /// Converts an integer to a `NonZeroRatio`
    #[must_use]
    pub const fn integer(integer: NonZeroU32) -> NonZeroRatio {
        NonZeroRatio {
            numerator: integer,
            denominator: ONE,
        }
    }

    /// Constructs the ratio 1 / `integer`
    #[must_use]
    pub const fn reciprocal_of(integer: NonZeroU32) -> NonZeroRatio {
        NonZeroRatio {
            numerator: ONE,
            denominator: integer,
        }
    }

    /// Returns the reciprocal of `self`
    #[must_use]
    pub const fn reciprocal(self) -> NonZeroRatio {
        NonZeroRatio {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    /// Converts `self` to a `Ratio`
    #[must_use]
    pub fn get(self) -> Ratio {
        Ratio {
            numerator: self.numerator.get(),
            denominator: self.denominator,
        }
    }

    /// Converts a `Ratio` to a `NonZeroRatio` if it is not zero
    #[must_use]
    pub fn from_ratio(ratio: Ratio) -> Option<NonZeroRatio> {
        Some(NonZeroRatio {
            numerator: NonZeroU32::new(ratio.numerator)?,
            denominator: ratio.denominator,
        })
    }

    pub(super) fn approximate_big(
        mut numerator: NonZeroU128,
        mut denominator: NonZeroU128,
    ) -> NonZeroRatio {
        let fallback = match numerator.cmp(&denominator) {
            Ordering::Less => NonZeroRatio::MIN,
            Ordering::Equal => return NonZeroRatio::ONE,
            Ordering::Greater => NonZeroRatio::MAX,
        };

        loop {
            if let Ok(numerator) = NonZeroU32::try_from(numerator) {
                if let Ok(denominator) = NonZeroU32::try_from(denominator) {
                    return NonZeroRatio::new(numerator, denominator);
                }
            }

            let Some(new_numerator) = NonZeroU128::new(numerator.get() >> 1) else {
                return fallback;
            };
            let Some(new_denominator) = NonZeroU128::new(denominator.get() >> 1) else {
                return fallback;
            };

            numerator = new_numerator;
            denominator = new_denominator;
        }
    }
}

impl PartialOrd<Self> for NonZeroRatio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NonZeroRatio {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
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
