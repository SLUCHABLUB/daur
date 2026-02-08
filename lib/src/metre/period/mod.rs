//! Items pertaining to [`Period`].

mod non_zero;

pub use non_zero::NonZeroPeriod;

use crate::metre::Duration;
use crate::metre::Instant;
use std::cmp::max;
use std::cmp::min;
use std::ops::Range;

/// A period of musical time.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
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

    /// Constructs a new period from a starting and an ending point.
    /// If the ending point is before the starting one, [`None`] is returned.
    #[must_use]
    pub fn from_endpoints(start: Instant, end: Instant) -> Option<Period> {
        if end < start {
            return None;
        }

        Some(Period {
            start,
            duration: end - start,
        })
    }

    /// Returns the intersection between the two periods.
    /// If the periods do not intersect, [`None`] is returned.
    #[must_use]
    pub fn intersection(first: Period, second: Period) -> Option<Period> {
        Period::from_endpoints(
            max(first.start, second.start),
            min(first.end(), second.end()),
        )
    }

    /// Converts the period to a range of instants.
    fn range(self) -> Range<Instant> {
        Range {
            start: self.start,
            end: self.end(),
        }
    }

    /// Whether the period contains the specified instant.
    #[must_use]
    pub fn contains(self, instant: Instant) -> bool {
        self.range().contains(&instant)
    }
}
