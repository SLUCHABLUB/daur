use crate::time::duration::NonZeroDuration;
use crate::time::instant::Instant;
use std::collections::Bound;
use std::ops::{RangeBounds, RangeFrom};

/// An `Instant` distinct from  the starting point
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct NonZeroInstant {
    /// The duration since the starting point
    pub since_start: NonZeroDuration,
}

impl NonZeroInstant {
    /// Converts `self` to an `Instant`
    #[must_use]
    pub fn get(self) -> Instant {
        Instant {
            since_start: self.since_start.get(),
        }
    }

    /// Converts an `Instant` to a `NonZeroInstant` if it is not the starting point
    #[must_use]
    pub fn from_instant(instant: Instant) -> Option<NonZeroInstant> {
        Some(NonZeroInstant {
            since_start: NonZeroDuration::from_duration(instant.since_start)?,
        })
    }
}

impl RangeBounds<NonZeroInstant> for RangeFrom<Instant> {
    fn start_bound(&self) -> Bound<&NonZeroInstant> {
        if self.start == Instant::START {
            Bound::Unbounded
        } else {
            let pointer: *const Instant = &self.start;
            let pointer = pointer.cast::<NonZeroInstant>();

            // Safety:
            // We have checked that self.start != 0 which is the invariant of `NonZeroInstant`
            // Tests also check that these have the same size
            let reference = unsafe { &*pointer };

            Bound::Included(reference)
        }
    }

    fn end_bound(&self) -> Bound<&NonZeroInstant> {
        Bound::Unbounded
    }
}
