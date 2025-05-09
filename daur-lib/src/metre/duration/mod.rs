mod non_zero;

pub use non_zero::NonZeroDuration;

use crate::ratio::{NonZeroRatio, Ratio};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

/// A musical duration
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Duration {
    /// The number of whole-note durations
    pub whole_notes: Ratio,
}

impl Duration {
    /// No time
    pub const ZERO: Duration = Duration {
        whole_notes: Ratio::ZERO,
    };
}

// TODO: derive
impl Add for Duration {
    type Output = Duration;

    fn add(mut self, rhs: Duration) -> Duration {
        self += rhs;
        self
    }
}

// TODO: derive
impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        self.whole_notes += rhs.whole_notes;
    }
}

// TODO: derive
impl Sub for Duration {
    type Output = Duration;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

// TODO: derive
impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        self.whole_notes -= rhs.whole_notes;
    }
}

// TODO: derive
impl Mul<Ratio> for Duration {
    type Output = Duration;

    fn mul(mut self, rhs: Ratio) -> Duration {
        self *= rhs;
        self
    }
}

// TODO: derive
impl MulAssign<Ratio> for Duration {
    fn mul_assign(&mut self, rhs: Ratio) {
        self.whole_notes *= rhs;
    }
}

// TODO: derive
impl Div<NonZeroDuration> for Duration {
    type Output = Ratio;

    fn div(self, rhs: NonZeroDuration) -> Ratio {
        self.whole_notes / rhs.whole_notes
    }
}

// TODO: derive
impl Div<NonZeroRatio> for Duration {
    type Output = Duration;

    fn div(self, rhs: NonZeroRatio) -> Duration {
        Duration {
            whole_notes: self.whole_notes / rhs,
        }
    }
}
