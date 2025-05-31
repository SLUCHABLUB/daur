use crate::ratio::util::lcm;
use crate::{NonZeroRatio, Ratio};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// --- INFIX OPERATIONS ---

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

impl Div<NonZeroRatio> for Ratio {
    type Output = Ratio;

    fn div(self, rhs: NonZeroRatio) -> Ratio {
        #![expect(clippy::suspicious_arithmetic_impl, reason = "we take the reciprocal")]
        self * rhs.reciprocal().get()
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl AddAssign for Ratio {
    fn add_assign(&mut self, rhs: Ratio) {
        *self = *self + rhs;
    }
}

impl SubAssign for Ratio {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Ratio {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign<NonZeroRatio> for Ratio {
    fn div_assign(&mut self, rhs: NonZeroRatio) {
        *self = *self / rhs;
    }
}

// --- NON-ZERO OPERATIONS ---

impl Div for NonZeroRatio {
    type Output = NonZeroRatio;

    fn div(self, rhs: Self) -> Self::Output {
        #[expect(
            clippy::unwrap_used,
            reason = "the numerator is non-zero; therefore, the result will be too"
        )]
        NonZeroRatio::from_ratio(self.get() / rhs).unwrap()
    }
}

impl DivAssign for NonZeroRatio {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
