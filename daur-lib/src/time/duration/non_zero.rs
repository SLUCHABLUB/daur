use crate::ratio::NonZeroRatio;
use crate::time::duration::Duration;
use std::ops::{Div, DivAssign};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct NonZeroDuration {
    pub whole_notes: NonZeroRatio,
}

impl NonZeroDuration {
    pub const QUARTER: NonZeroDuration = NonZeroDuration {
        whole_notes: NonZeroRatio::QUARTER,
    };

    pub fn get(self) -> Duration {
        Duration {
            whole_notes: self.whole_notes.get(),
        }
    }

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
