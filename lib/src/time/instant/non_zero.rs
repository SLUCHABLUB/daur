use crate::time::Instant;
use crate::time::NonZeroDuration;

/// An [instant](super::Instant) that is strictly after the starting point.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
}
