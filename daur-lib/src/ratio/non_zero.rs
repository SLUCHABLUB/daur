use crate::ratio::Ratio;
use crate::ratio::util::make_coprime;
use getset::CopyGetters;
use non_zero::non_zero;
use std::cmp::Ordering;
use std::num::{NonZeroU64, NonZeroU128};

/// A non-zero [ratio](Ratio)
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, CopyGetters)]
pub struct NonZeroRatio {
    /// The numerator.
    #[get_copy = "pub"]
    numerator: NonZeroU64,
    /// The denominator.
    #[get_copy = "pub"]
    denominator: NonZeroU64,
}

impl NonZeroRatio {
    /// 1 / 4
    pub const QUARTER: NonZeroRatio = NonZeroRatio::reciprocal_of(non_zero!(4));

    /// 1
    pub const ONE: NonZeroRatio = NonZeroRatio::integer(non_zero!(1));

    const MIN: NonZeroRatio = NonZeroRatio::reciprocal_of(NonZeroU64::MAX);
    const MAX: NonZeroRatio = NonZeroRatio::integer(NonZeroU64::MAX);

    /// Creates a new ratio from a numerator and a denominator.
    #[must_use]
    pub fn new(numerator: NonZeroU64, denominator: NonZeroU64) -> NonZeroRatio {
        // The non-zero types do not implement `num::Integer`.
        // Therefore, a `num::rational::Ratio` thereof cannot be reduced
        // since it requires comparison with zero.

        let [numerator, denominator] = make_coprime(numerator, denominator);

        NonZeroRatio {
            numerator,
            denominator,
        }
    }

    /// Converts an integer to a ratio.
    #[must_use]
    pub const fn integer(integer: NonZeroU64) -> NonZeroRatio {
        NonZeroRatio {
            numerator: integer,
            denominator: non_zero!(1),
        }
    }

    /// Constructs a ratio from an integer by taking its reciprocal.
    #[must_use]
    pub const fn reciprocal_of(integer: NonZeroU64) -> NonZeroRatio {
        NonZeroRatio {
            numerator: non_zero!(1),
            denominator: integer,
        }
    }

    /// Returns the reciprocal of the ratio.
    #[must_use]
    pub const fn reciprocal(self) -> NonZeroRatio {
        NonZeroRatio {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    /// Converts the ratio to a [zeroable one](Ratio).
    #[must_use]
    pub fn get(self) -> Ratio {
        Ratio {
            numerator: self.numerator.get(),
            denominator: self.denominator,
        }
    }

    /// Converts a ratio to a non-zero one if it is not zero.
    #[must_use]
    pub fn from_ratio(ratio: Ratio) -> Option<NonZeroRatio> {
        Some(NonZeroRatio {
            numerator: NonZeroU64::new(ratio.numerator)?,
            denominator: ratio.denominator,
        })
    }

    /// Calculates the ceiling of the ratio.
    #[must_use]
    pub fn ceil(self) -> NonZeroU64 {
        NonZeroU64::new(self.get().ceil()).unwrap_or(non_zero!(1))
    }

    /// Returns a ratio representing the ceiling of the ratio.
    #[must_use]
    pub fn ceiled(self) -> NonZeroRatio {
        NonZeroRatio::integer(self.ceil())
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
            if let Ok(numerator) = NonZeroU64::try_from(numerator)
                && let Ok(denominator) = NonZeroU64::try_from(denominator)
            {
                return NonZeroRatio::new(numerator, denominator);
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

impl PartialOrd for NonZeroRatio {
    fn partial_cmp(&self, other: &NonZeroRatio) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NonZeroRatio {
    fn cmp(&self, other: &NonZeroRatio) -> Ordering {
        self.get().cmp(&other.get())
    }
}

impl From<NonZeroRatio> for Ratio {
    fn from(value: NonZeroRatio) -> Self {
        value.get()
    }
}

impl<T: Into<NonZeroU64>> From<T> for NonZeroRatio {
    fn from(value: T) -> NonZeroRatio {
        NonZeroRatio::integer(value.into())
    }
}
