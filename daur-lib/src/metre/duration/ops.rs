use crate::metre::{Duration, NonZeroDuration};
use crate::{NonZeroRatio, Ratio};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl<D: Into<Duration>> Add<D> for Duration {
    type Output = Duration;

    fn add(mut self, rhs: D) -> Duration {
        self += rhs;
        self
    }
}

impl<D: Into<Duration>> Sub<D> for Duration {
    type Output = Duration;

    fn sub(mut self, rhs: D) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<R: Into<Ratio>> Mul<R> for Duration {
    type Output = Duration;

    fn mul(mut self, rhs: R) -> Duration {
        self *= rhs;
        self
    }
}

impl<N: Into<NonZeroRatio>> Div<N> for Duration {
    type Output = Duration;

    fn div(mut self, rhs: N) -> Duration {
        self /= rhs;
        self
    }
}

// -- ASSIGNMENT OPERATIONS ---

impl<D: Into<Duration>> AddAssign<D> for Duration {
    fn add_assign(&mut self, rhs: D) {
        self.whole_notes += rhs.into().whole_notes;
    }
}

impl<D: Into<Duration>> SubAssign<D> for Duration {
    fn sub_assign(&mut self, rhs: D) {
        self.whole_notes -= rhs.into().whole_notes;
    }
}

impl<R: Into<Ratio>> MulAssign<R> for Duration {
    fn mul_assign(&mut self, rhs: R) {
        self.whole_notes *= rhs;
    }
}

impl<N: Into<NonZeroRatio>> DivAssign<N> for Duration {
    fn div_assign(&mut self, rhs: N) {
        self.whole_notes /= rhs;
    }
}

// --- STRICTLY INFIX OPERATIONS ---

impl Div<NonZeroDuration> for Duration {
    type Output = Ratio;

    fn div(self, rhs: NonZeroDuration) -> Ratio {
        self.whole_notes / rhs.whole_notes
    }
}
