use crate::time::duration::NonZeroDuration;
use crate::time::instant::Instant;
use std::collections::Bound;
use std::ops::{RangeBounds, RangeFrom};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct NonZeroInstant {
    pub since_start: NonZeroDuration,
}

impl NonZeroInstant {
    pub fn get(self) -> Instant {
        Instant {
            since_start: self.since_start.get(),
        }
    }

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
            // TODO: turn this into a test
            debug_assert_eq!(
                size_of::<Instant>(),
                64,
                "size of Instant did not match expectation"
            );
            debug_assert_eq!(
                size_of::<NonZeroInstant>(),
                64,
                "size of Instant did not match expectation"
            );

            let pointer: *const Instant = &self.start;
            let pointer = pointer.cast::<NonZeroInstant>();

            // Safety: We have checked that self.start != 0 which is the invariant of `NonZeroInstant`
            let reference = unsafe { &*pointer };

            Bound::Included(reference)
        }
    }

    fn end_bound(&self) -> Bound<&NonZeroInstant> {
        Bound::Unbounded
    }
}
