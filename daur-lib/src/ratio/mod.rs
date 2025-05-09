mod non_zero;
mod util;

pub use non_zero::NonZeroRatio;

use crate::ratio::util::lcm;
use ::non_zero::non_zero;
use core::cmp::Ordering;
use core::fmt;
use core::fmt::{Display, Formatter};
use core::num::{FpCategory, NonZeroU64, NonZeroU128};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use getset::CopyGetters;
use saturating_cast::SaturatingCast as _;

/// A rational number with saturating semantics.
/// When operations would result in a non-representable value, the result is an approximation.
// INVARIANT: `numerator` and `denominator` are co-prime
// due to this we can derive `Eq` and `PartialEq`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, CopyGetters)]
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

    const EPSILON: Ratio = Ratio::reciprocal_of(NonZeroU64::MAX);
    const MAX: Ratio = Ratio::integer(u64::MAX);

    /// Creates a new ratio from a numerator and denominator.
    #[must_use]
    pub fn new(numerator: u64, denominator: NonZeroU64) -> Ratio {
        let Some(numerator) = NonZeroU64::new(numerator) else {
            return Self::ZERO;
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

    /// Calculates the ceiling of the ratio
    #[must_use]
    pub fn ceil(self) -> u64 {
        let quotient = self.numerator / self.denominator;
        let remainder = self.numerator % self.denominator;

        if remainder == 0 {
            quotient
        } else {
            quotient.saturating_add(1)
        }
    }

    /// Returns a ratio representing the ceiling of the ratio
    #[must_use]
    pub fn ceiled(self) -> Ratio {
        Ratio::integer(self.ceil())
    }

    /// Calculates the floor of the ratio
    #[must_use]
    pub fn floor(self) -> u64 {
        self.numerator / self.denominator
    }

    /// Returns a ratio representing the floor of the ratio.
    #[must_use]
    pub fn floored(self) -> Ratio {
        Ratio::integer(self.floor())
    }

    /// Rounds the ratio to an integer.
    #[must_use]
    pub fn round(self) -> u64 {
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

            if mean_guess == low_guess || mean_guess == high_guess {
                return mean_guess;
            }

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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
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
    fn default() -> Self {
        Ratio::ZERO
    }
}

impl<T: Into<u64>> From<T> for Ratio {
    fn from(value: T) -> Self {
        Ratio::integer(value.into())
    }
}

impl Add for Ratio {
    type Output = Ratio;

    fn add(self, rhs: Ratio) -> Ratio {
        let (lhs_numerator, lhs_denominator) = self.big_raw();
        let (rhs_numerator, rhs_denominator) = rhs.big_raw();

        let lcm = lcm(lhs_denominator, rhs_denominator);

        let lhs = lhs_numerator.saturating_mul(lcm.get() / lhs_denominator);
        let rhs = rhs_numerator.saturating_mul(lcm.get() / rhs_denominator);

        #[expect(clippy::arithmetic_side_effects, reason = "we encapsulate in u128")]
        Ratio::approximate_big(lhs + rhs, lcm)
    }
}

impl AddAssign for Ratio {
    fn add_assign(&mut self, rhs: Ratio) {
        *self = *self + rhs;
    }
}

impl Sub for Ratio {
    type Output = Ratio;

    fn sub(self, rhs: Self) -> Self::Output {
        let (lhs_numerator, lhs_denominator) = self.big_raw();
        let (rhs_numerator, rhs_denominator) = rhs.big_raw();

        let lcm = lcm(lhs_denominator, rhs_denominator);

        let lhs = lhs_numerator.saturating_mul(lcm.get() / lhs_denominator);
        let rhs = rhs_numerator.saturating_mul(lcm.get() / rhs_denominator);

        Ratio::approximate_big(lhs.saturating_sub(rhs), lcm)
    }
}

impl SubAssign for Ratio {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Ratio {
    type Output = Ratio;

    fn mul(self, rhs: Ratio) -> Ratio {
        let (lhs_numerator, lhs_denominator) = self.big_raw();
        let (rhs_numerator, rhs_denominator) = rhs.big_raw();

        Ratio::approximate_big(
            lhs_numerator.saturating_mul(rhs_numerator),
            lhs_denominator.saturating_mul(rhs_denominator),
        )
    }
}

impl MulAssign for Ratio {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div<NonZeroRatio> for Ratio {
    type Output = Ratio;

    fn div(self, rhs: NonZeroRatio) -> Ratio {
        #![expect(clippy::suspicious_arithmetic_impl, reason = "we take the reciprocal")]
        self * rhs.reciprocal().get()
    }
}

impl DivAssign<NonZeroRatio> for Ratio {
    fn div_assign(&mut self, rhs: NonZeroRatio) {
        *self = *self / rhs;
    }
}
