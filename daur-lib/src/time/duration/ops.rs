use crate::time::{Duration, NonZeroDuration};
use crate::{NonZeroRatio, Ratio};
use std::num::NonZeroU128;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// --- INFIX OPERATIONS ---

impl<D: Into<Duration>> Add<D> for Duration {
    type Output = Duration;

    fn add(self, rhs: D) -> Duration {
        Duration {
            nanoseconds: self.nanoseconds.saturating_add(rhs.into().nanoseconds),
        }
    }
}

impl<D: Into<Duration>> Sub<D> for Duration {
    type Output = Duration;

    fn sub(self, rhs: D) -> Duration {
        Duration {
            nanoseconds: self.nanoseconds.saturating_sub(rhs.into().nanoseconds),
        }
    }
}

impl<R: Into<Ratio>> Mul<R> for Duration {
    type Output = Duration;

    fn mul(self, rhs: R) -> Duration {
        let rhs = rhs.into();

        let nanoseconds = u128::from(self.nanoseconds);
        let numerator = u128::from(rhs.numerator());
        let denominator = NonZeroU128::from(rhs.denominator());

        // TODO: round
        #[expect(clippy::arithmetic_side_effects, reason = "we encapsulate in u128")]
        #[expect(clippy::integer_division, reason = "see TODO")]
        let nanoseconds = nanoseconds * numerator / denominator;
        let nanoseconds = u64::try_from(nanoseconds).unwrap_or(u64::MAX);

        Duration { nanoseconds }
    }
}

impl<N: Into<NonZeroRatio>> Div<N> for Duration {
    type Output = Duration;

    fn div(self, rhs: N) -> Duration {
        #![expect(clippy::suspicious_arithmetic_impl, reason = "we take the reciprocal")]
        self * rhs.into().reciprocal().get()
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl<D: Into<Duration>> AddAssign<D> for Duration {
    fn add_assign(&mut self, rhs: D) {
        *self = *self + rhs;
    }
}

impl<D: Into<Duration>> SubAssign<D> for Duration {
    fn sub_assign(&mut self, rhs: D) {
        *self = *self - rhs;
    }
}

impl<R: Into<Ratio>> MulAssign<R> for Duration {
    fn mul_assign(&mut self, rhs: R) {
        *self = *self * rhs;
    }
}

impl<N: Into<NonZeroRatio>> DivAssign<N> for Duration {
    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs;
    }
}

// --- NON-CLOSED OPERATIONS ---

impl Div<NonZeroDuration> for Duration {
    type Output = Ratio;

    fn div(self, rhs: NonZeroDuration) -> Ratio {
        Ratio::approximate_big(
            u128::from(self.nanoseconds),
            NonZeroU128::from(rhs.nanoseconds),
        )
    }
}
