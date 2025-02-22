mod non_zero;
mod util;

pub use non_zero::NonZeroRatio;
use std::cmp::Ordering;

use crate::ratio::util::lcm;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::{FpCategory, NonZeroU128, NonZeroU32};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

const ONE: NonZeroU32 = NonZeroU32::MIN;
const TWO: NonZeroU32 = ONE.saturating_add(1);
const FOUR: NonZeroU32 = TWO.saturating_pow(2);

/// A rational number with saturating semantics.
/// When operations would result in a non-representable value, the result is an approximation.
// INVARIANT: `numerator` and `denominator` are co-prime
// due to this we can derive `Eq` and `PartialEq`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Ratio {
    numerator: u32,
    denominator: NonZeroU32,
}

impl Ratio {
    /// 0
    pub const ZERO: Ratio = Ratio::integer(0);

    /// 1 / 2
    pub const HALF: Ratio = Ratio::reciprocal_of(TWO);

    /// 1 / 4
    pub const QUARTER: Ratio = Ratio::reciprocal_of(FOUR);

    /// 1
    pub const ONE: Ratio = Ratio::integer(1);

    const MAX: Ratio = Ratio::integer(u32::MAX);

    /// Creates a new `Ratio` representing `numerator` / `denominator`
    #[must_use]
    pub fn new(numerator: u32, denominator: NonZeroU32) -> Ratio {
        let Some(numerator) = NonZeroU32::new(numerator) else {
            return Self::ZERO;
        };

        NonZeroRatio::new(numerator, denominator).get()
    }

    /// Converts an integer to a `Ratio`
    #[must_use]
    pub const fn integer(integer: u32) -> Ratio {
        Ratio {
            numerator: integer,
            denominator: ONE,
        }
    }

    /// Constructs the ratio 1 / `integer`
    #[must_use]
    pub const fn reciprocal_of(integer: NonZeroU32) -> Ratio {
        Ratio {
            numerator: 1,
            denominator: integer,
        }
    }

    /// Calculates the ceiling of the ratio
    #[must_use]
    pub fn ceil(self) -> u32 {
        let quotient = self.numerator / self.denominator;
        let remainder = self.numerator % self.denominator;

        if remainder == 0 {
            quotient
        } else {
            quotient.saturating_add(1)
        }
    }

    /// Returns the ratio representing the ceiling of `self`
    #[must_use]
    pub fn ceiled(self) -> Ratio {
        Ratio::integer(self.ceil())
    }

    /// Rounds `self` to an integer
    #[must_use]
    pub fn round(self) -> u32 {
        let quotient = self.numerator / self.denominator;
        let remainder = self.numerator % self.denominator;

        let fractional_part = Ratio::new(remainder, self.denominator);

        if fractional_part < Ratio::HALF {
            quotient
        } else {
            quotient.saturating_add(1)
        }
    }

    /// Rounds `self`
    #[must_use]
    pub fn rounded(self) -> Ratio {
        Ratio::integer(self.round())
    }

    /// Approximates a float as a `Ratio`
    #[must_use]
    pub fn approximate(float: f64) -> Ratio {
        #![expect(clippy::cast_sign_loss, reason = "we check sign")]
        #![expect(
            clippy::cast_possible_truncation,
            reason = "values are converted to integers and checked against MAX"
        )]

        const MAX: f64 = Ratio::MAX.to_float();
        const EPSILON: f64 = Ratio::MAX.to_float();

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

        let mut low_guess = Ratio::integer(float.floor() as u32);
        let mut high_guess = Ratio::integer(float.ceil() as u32);

        loop {
            let mean_guess = (low_guess + high_guess) * Ratio::HALF;

            match float.total_cmp(&mean_guess.to_float()) {
                Ordering::Less => high_guess = mean_guess,
                Ordering::Equal => return mean_guess,
                Ordering::Greater => low_guess = mean_guess,
            }
        }
    }

    /// Approximates `self` as a float
    #[must_use]
    pub const fn to_float(self) -> f64 {
        self.numerator as f64 / self.denominator.get() as f64
    }

    fn approximate_big(denominator: u128, reciprocal: NonZeroU128) -> Ratio {
        let Some(denominator) = NonZeroU128::new(denominator) else {
            return Ratio::ZERO;
        };

        NonZeroRatio::approximate_big(denominator, reciprocal).get()
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
        #![expect(clippy::arithmetic_side_effects, reason = "we cast to u64 first")]
        Ord::cmp(
            &(u64::from(self.numerator) * u64::from(other.denominator.get())),
            &(u64::from(other.numerator) * u64::from(self.denominator.get())),
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

impl Add for Ratio {
    type Output = Ratio;

    fn add(self, rhs: Ratio) -> Ratio {
        let (lhs_numerator, lhs_denominator) = self.big_raw();
        let (rhs_numerator, rhs_denominator) = rhs.big_raw();

        let lcm = lcm(lhs_denominator, rhs_denominator);

        let lhs = lhs_numerator.saturating_mul(lcm.get());
        let rhs = rhs_numerator.saturating_mul(lcm.get());

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

        let lhs = lhs_numerator.saturating_mul(lcm.get());
        let rhs = rhs_numerator.saturating_mul(lcm.get());

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
