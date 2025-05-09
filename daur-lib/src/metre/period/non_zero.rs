use crate::metre::{Instant, NonZeroDuration, Period};

/// A period of musical time with a non-zero duration.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NonZeroPeriod {
    /// The start of the period.
    pub start: Instant,
    /// The duration of the period.
    pub duration: NonZeroDuration,
}

impl NonZeroPeriod {
    /// Converts the period to a [zeroable one](Period).
    #[must_use]
    pub fn get(self) -> Period {
        Period {
            start: self.start,
            duration: self.duration.get(),
        }
    }

    /// Converts a period to a non-zero one if it is not zero.
    #[must_use]
    pub fn from_period(period: Period) -> Option<NonZeroPeriod> {
        Some(NonZeroPeriod {
            start: period.start,
            duration: NonZeroDuration::from_duration(period.duration)?,
        })
    }

    /// Constructs a new period from a starting and an ending point.
    /// If the ending point is before, or equal to, the starting one, [`None`] is returned.
    #[must_use]
    pub fn from_endpoints(start: Instant, end: Instant) -> Option<NonZeroPeriod> {
        NonZeroPeriod::from_period(Period::from_endpoints(start, end)?)
    }
}
