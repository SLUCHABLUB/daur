use crate::time::Duration;
use crate::NonZeroRatio;
use std::ops::{Div, DivAssign};

/// A non-zero `Duration`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonZeroDuration {
    /// The number of whole-note durations
    pub whole_notes: NonZeroRatio,
}

impl NonZeroDuration {
    /// The duration of a quarter note
    pub const QUARTER: NonZeroDuration = NonZeroDuration {
        whole_notes: NonZeroRatio::QUARTER,
    };

    /// Converts `self` to a `Duration`
    #[must_use]
    pub fn get(self) -> Duration {
        Duration {
            whole_notes: self.whole_notes.get(),
        }
    }

    /// Converts a `Duration` to a `NonZeroDuration` is it is not zero
    #[must_use]
    pub fn from_duration(duration: Duration) -> Option<NonZeroDuration> {
        Some(NonZeroDuration {
            whole_notes: NonZeroRatio::from_ratio(duration.whole_notes)?,
        })
    }
}

impl Div for NonZeroDuration {
    type Output = NonZeroRatio;

    fn div(self, rhs: Self) -> Self::Output {
        self.whole_notes / rhs.whole_notes
    }
}

impl Div<NonZeroRatio> for NonZeroDuration {
    type Output = NonZeroDuration;

    fn div(mut self, rhs: NonZeroRatio) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<NonZeroRatio> for NonZeroDuration {
    fn div_assign(&mut self, rhs: NonZeroRatio) {
        self.whole_notes /= rhs;
    }
}
