use crate::metre::{Duration, NonZeroDuration};
use crate::{NonZeroRatio, Ratio};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl Add for Duration {
    type Output = Duration;

    fn add(mut self, rhs: Duration) -> Duration {
        self += rhs;
        self
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Mul<Ratio> for Duration {
    type Output = Duration;

    fn mul(mut self, rhs: Ratio) -> Duration {
        self *= rhs;
        self
    }
}

impl Div<NonZeroRatio> for Duration {
    type Output = Duration;

    fn div(mut self, rhs: NonZeroRatio) -> Duration {
        self /= rhs;
        self
    }
}

// -- ASSIGNMENT OPERATIONS ---

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        self.whole_notes += rhs.whole_notes;
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        self.whole_notes -= rhs.whole_notes;
    }
}

impl MulAssign<Ratio> for Duration {
    fn mul_assign(&mut self, rhs: Ratio) {
        self.whole_notes *= rhs;
    }
}

impl DivAssign<NonZeroRatio> for Duration {
    fn div_assign(&mut self, rhs: NonZeroRatio) {
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
