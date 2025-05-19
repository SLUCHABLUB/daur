use crate::NonZeroRatio;
use crate::metre::Duration;
use std::ops::{Div, DivAssign};

/// A non-zero duration.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonZeroDuration {
    /// The number of whole-note durations
    pub whole_notes: NonZeroRatio,
}

impl NonZeroDuration {
    /// The duration of a quarter note.
    pub const QUARTER: NonZeroDuration = NonZeroDuration {
        whole_notes: NonZeroRatio::QUARTER,
    };

    /// Converts the duration to a [zeroable one](Duration).
    #[must_use]
    pub fn get(self) -> Duration {
        Duration {
            whole_notes: self.whole_notes.get(),
        }
    }

    /// Converts a duration to a non-zero one if it is not zero.
    #[must_use]
    pub fn from_duration(duration: Duration) -> Option<NonZeroDuration> {
        Some(NonZeroDuration {
            whole_notes: NonZeroRatio::from_ratio(duration.whole_notes)?,
        })
    }
}

// TODO: derive
impl Div for NonZeroDuration {
    type Output = NonZeroRatio;

    fn div(self, rhs: Self) -> Self::Output {
        self.whole_notes / rhs.whole_notes
    }
}

// TODO: derive
impl Div<NonZeroRatio> for NonZeroDuration {
    type Output = NonZeroDuration;

    fn div(mut self, rhs: NonZeroRatio) -> Self::Output {
        self /= rhs;
        self
    }
}

// TODO: derive
impl DivAssign<NonZeroRatio> for NonZeroDuration {
    fn div_assign(&mut self, rhs: NonZeroRatio) {
        self.whole_notes /= rhs;
    }
}
