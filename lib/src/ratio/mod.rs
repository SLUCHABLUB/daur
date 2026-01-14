//! Items pertaining to [`Ratio`].

mod non_zero;
mod ops;
mod serial;
mod util;

pub use non_zero::NonZeroRatio;

use ::non_zero::non_zero;
use getset::CopyGetters;
use saturating_cast::SaturatingCast as _;
use serde::Deserialize;
use serde::Serialize;
use serial::Serial;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::FpCategory;
use std::num::NonZeroU64;
use std::num::NonZeroU128;

/// A rational number with saturating semantics.
/// When operations would result in a non-representable value, the result is an approximation.
// INVARIANT: `numerator` and `denominator` are co-prime
// due to this we can derive `Eq` and `PartialEq`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, CopyGetters, Serialize, Deserialize)]
#[serde(try_from = "Serial", into = "Serial")]
pub struct Ratio {
    /// The numerator.
    #[get_copy = "pub"]
    numerator: u64,
    /// The numerator.
    #[get_copy = "pub"]
    denominator: NonZeroU64,
}

impl Ratio {
    /// 0
    pub const ZERO: Ratio = Ratio::integer(0);

    /// 1 / 2
    pub const HALF: Ratio = Ratio::reciprocal_of(non_zero!(2));

    /// 1 / 4
    pub const QUARTER: Ratio = Ratio::reciprocal_of(non_zero!(4));

    /// 1
    pub const ONE: Ratio = Ratio::integer(1);

    /// The minimum non-zero ratio.
    const EPSILON: Ratio = Ratio::reciprocal_of(NonZeroU64::MAX);
    /// The maximum ratio.
    const MAX: Ratio = Ratio::integer(u64::MAX);

    /// Creates a new ratio from a numerator and denominator.
    #[must_use]
    pub fn new(numerator: u64, denominator: NonZeroU64) -> Ratio {
        let Some(numerator) = NonZeroU64::new(numerator) else {
            return Ratio::ZERO;
        };

        NonZeroRatio::new(numerator, denominator).get()
    }

    /// Converts an integer to a ratio.
    #[must_use]
    pub const fn integer(integer: u64) -> Ratio {
        Ratio {
            numerator: integer,
            denominator: non_zero!(1),
        }
    }

    /// Constructs the ratio from an integer by taking its reciprocal.
    #[must_use]
    pub const fn reciprocal_of(integer: NonZeroU64) -> Ratio {
        Ratio {
            numerator: 1,
            denominator: integer,
        }
    }

    /// Calculates the ceiling of the ratio.
    #[must_use]
    pub fn ceiling(self) -> u64 {
        #[expect(clippy::integer_division, reason = "we also take the remainder")]
        let quotient = self.numerator / self.denominator;
        let remainder = self.numerator % self.denominator;

        if remainder == 0 {
            quotient
        } else {
            quotient.saturating_add(1)
        }
    }

    /// Calculates the floor of the ratio
    #[must_use]
    pub fn floor(self) -> u64 {
        #![expect(clippy::integer_division, reason = "we want the floor")]
        self.numerator / self.denominator
    }

    /// Rounds the ratio to an integer.
    #[must_use]
    pub fn round(self) -> u64 {
        #[expect(clippy::integer_division, reason = "we also take the remainder")]
        let quotient = self.numerator / self.denominator;
        let remainder = self.numerator % self.denominator;

        let fractional_part = Ratio::new(remainder, self.denominator);

        if fractional_part < Ratio::HALF {
            quotient
        } else {
            quotient.saturating_add(1)
        }
    }

    /// Rounds the ratio.
    #[must_use]
    pub fn rounded(self) -> Ratio {
        Ratio::integer(self.round())
    }

    /// Like [`Ratio::round`] but rounds 1/2 towards zero.
    #[must_use]
    pub fn round_half_down(self) -> u64 {
        #[expect(clippy::integer_division, reason = "we also take the remainder")]
        let quotient = self.numerator / self.denominator;
        let remainder = self.numerator % self.denominator;

        let fractional_part = Ratio::new(remainder, self.denominator);

        if fractional_part <= Ratio::HALF {
            quotient
        } else {
            quotient.saturating_add(1)
        }
    }

    /// Like [`Ratio::rounded`] but rounds 1/2 towards zero.
    #[must_use]
    pub fn rounded_half_down(self) -> Ratio {
        Ratio::integer(self.round_half_down())
    }

    /// Approximates a float as a ratio.
    #[must_use]
    pub fn approximate(float: f64) -> Ratio {
        #![expect(clippy::cast_sign_loss, reason = "we check the sign")]
        #![expect(
            clippy::cast_possible_truncation,
            reason = "values are converted to integers and checked against MAX"
        )]

        const MAX: f64 = Ratio::MAX.to_float();
        const EPSILON: f64 = Ratio::EPSILON.to_float();

        if float.is_sign_negative() {
            return Ratio::ZERO;
        }

        match float.classify() {
            FpCategory::Nan | FpCategory::Zero => return Ratio::ZERO,
            FpCategory::Infinite => return Ratio::MAX,
            FpCategory::Subnormal | FpCategory::Normal => (),
        }

        if float < EPSILON {
            return Ratio::ZERO;
        }
        if MAX < float {
            return Ratio::MAX;
        }

        let mut low_guess = Ratio::integer(float.floor() as u64);
        let mut high_guess = Ratio::integer(float.ceil() as u64);

        loop {
            let mean_guess = (low_guess + high_guess) * Ratio::HALF;

            match float.total_cmp(&mean_guess.to_float()) {
                Ordering::Less => high_guess = mean_guess,
                Ordering::Equal => return mean_guess,
                Ordering::Greater => low_guess = mean_guess,
            }
        }
    }

    /// Approximates a [`usize`].
    #[must_use]
    pub fn from_usize(value: usize) -> Ratio {
        Ratio::integer(value.saturating_cast())
    }

    /// Approximates the ratio as a float.
    #[must_use]
    pub const fn to_float(self) -> f64 {
        #![expect(clippy::cast_precision_loss, reason = "we approximate")]
        self.numerator as f64 / self.denominator.get() as f64
    }

    /// Rounds the ratio to a [`usize`].
    #[must_use]
    pub fn to_usize(self) -> usize {
        self.round().try_into().unwrap_or(usize::MAX)
    }

    /// Approximates a ratio between two 128-bit numbers.
    pub(crate) fn approximate_big(numerator: u128, denominator: NonZeroU128) -> Ratio {
        let Some(numerator) = NonZeroU128::new(numerator) else {
            return Ratio::ZERO;
        };

        NonZeroRatio::approximate_big(numerator, denominator).get()
    }

    /// Due to using lcm (multiplication) in addition to addition in addition (in extension),
    /// we need to use u128 as opposed to u64 for the result
    fn big_raw(self) -> (u128, NonZeroU128) {
        (
            u128::from(self.numerator),
            NonZeroU128::from(self.denominator),
        )
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Ratio) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Ratio) -> Ordering {
        #![expect(clippy::arithmetic_side_effects, reason = "we cast to u128 first")]
        Ord::cmp(
            &(u128::from(self.numerator) * u128::from(other.denominator.get())),
            &(u128::from(other.numerator) * u128::from(self.denominator.get())),
        )
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Default for Ratio {
    fn default() -> Ratio {
        Ratio::ZERO
    }
}

impl<T: Into<u64>> From<T> for Ratio {
    fn from(value: T) -> Ratio {
        Ratio::integer(value.into())
    }
}
