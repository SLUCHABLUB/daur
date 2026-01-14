//! Implementations of mathematical operations on [`Ratio`].

use crate::NonZeroRatio;
use crate::Ratio;
use crate::ratio::util::lowest_common_multiple;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

// --- INFIX OPERATIONS ---

impl<R: Into<Ratio>> Add<R> for Ratio {
    type Output = Ratio;

    fn add(self, rhs: R) -> Ratio {
        #![expect(
            clippy::integer_division,
            reason = "the lcm is divisible by its factors"
        )]

        let (lhs_numerator, lhs_denominator) = self.big_raw();
        let (rhs_numerator, rhs_denominator) = rhs.into().big_raw();

        let lcm = lowest_common_multiple(lhs_denominator, rhs_denominator);

        let lhs = lhs_numerator.saturating_mul(lcm.get() / lhs_denominator);
        let rhs = rhs_numerator.saturating_mul(lcm.get() / rhs_denominator);

        #[expect(clippy::arithmetic_side_effects, reason = "we encapsulate in u128")]
        Ratio::approximate_big(lhs + rhs, lcm)
    }
}

impl<R: Into<Ratio>> Sub<R> for Ratio {
    type Output = Ratio;

    fn sub(self, rhs: R) -> Ratio {
        #![expect(
            clippy::integer_division,
            reason = "the lcm is divisible by its factors"
        )]

        let (lhs_numerator, lhs_denominator) = self.big_raw();
        let (rhs_numerator, rhs_denominator) = rhs.into().big_raw();

        let lcm = lowest_common_multiple(lhs_denominator, rhs_denominator);

        let lhs = lhs_numerator.saturating_mul(lcm.get() / lhs_denominator);
        let rhs = rhs_numerator.saturating_mul(lcm.get() / rhs_denominator);

        Ratio::approximate_big(lhs.saturating_sub(rhs), lcm)
    }
}

impl<R: Into<Ratio>> Mul<R> for Ratio {
    type Output = Ratio;

    fn mul(self, rhs: R) -> Ratio {
        let (lhs_numerator, lhs_denominator) = self.big_raw();
        let (rhs_numerator, rhs_denominator) = rhs.into().big_raw();

        Ratio::approximate_big(
            lhs_numerator.saturating_mul(rhs_numerator),
            lhs_denominator.saturating_mul(rhs_denominator),
        )
    }
}

impl<N: Into<NonZeroRatio>> Div<N> for Ratio {
    type Output = Ratio;

    fn div(self, rhs: N) -> Ratio {
        #![expect(clippy::suspicious_arithmetic_impl, reason = "we take the reciprocal")]
        self * rhs.into().reciprocal().get()
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl<R: Into<Ratio>> AddAssign<R> for Ratio {
    fn add_assign(&mut self, rhs: R) {
        *self = *self + rhs;
    }
}

impl<R: Into<Ratio>> SubAssign<R> for Ratio {
    fn sub_assign(&mut self, rhs: R) {
        *self = *self - rhs;
    }
}

impl<R: Into<Ratio>> MulAssign<R> for Ratio {
    fn mul_assign(&mut self, rhs: R) {
        *self = *self * rhs;
    }
}

impl<N: Into<NonZeroRatio>> DivAssign<N> for Ratio {
    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs;
    }
}

// --- NON-ZERO OPERATIONS ---

impl<N: Into<NonZeroRatio>> Div<N> for NonZeroRatio {
    type Output = NonZeroRatio;

    fn div(self, rhs: N) -> NonZeroRatio {
        #[expect(
            clippy::unwrap_used,
            reason = "the numerator is non-zero; therefore, the result will be too"
        )]
        NonZeroRatio::from_ratio(self.get() / rhs).unwrap()
    }
}

impl<N: Into<NonZeroRatio>> DivAssign<N> for NonZeroRatio {
    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs;
    }
}
