use crate::metre::Instant;
use crate::metre::NonZeroDuration;
use serde::Deserialize;
use serde::Serialize;

/// An instant that is strictly after the starting point.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct NonZeroInstant {
    /// The duration since the starting point
    pub since_start: NonZeroDuration,
}

impl NonZeroInstant {
    /// Converts the instant to a [zeroable one](Instant).
    #[must_use]
    pub fn get(self) -> Instant {
        Instant {
            since_start: self.since_start.get(),
        }
    }

    /// Converts an instant to a non-zero one if it is not the starting point.
    #[must_use]
    pub fn from_instant(instant: Instant) -> Option<NonZeroInstant> {
        Some(NonZeroInstant {
            since_start: NonZeroDuration::from_duration(instant.since_start)?,
        })
    }
}
