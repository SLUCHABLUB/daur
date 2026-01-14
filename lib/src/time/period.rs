//! Items pertaining to [`Period`].

use crate::metre;
use crate::metre::Changing;
use crate::metre::TimeContext;
use crate::time::Duration;
use crate::time::Instant;
use std::ops::Div;

/// A period of real time.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Period {
    /// The start of the period.
    pub start: Instant,
    /// The duration of the period.
    pub duration: Duration,
}

impl Period {
    /// Returns the end of the period.
    #[must_use]
    pub fn end(&self) -> Instant {
        self.start + self.duration
    }
}

impl Div<&Changing<TimeContext>> for Period {
    type Output = metre::Period;

    fn div(self, rhs: &Changing<TimeContext>) -> metre::Period {
        let start = self.start / rhs;
        let end = self.end() / rhs;

        metre::Period {
            start,
            duration: end - start,
        }
    }
}
