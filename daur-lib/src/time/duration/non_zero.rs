use crate::time::Duration;
use std::num::NonZeroU64;

/// A non-zero [duration of real time](Duration).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonZeroDuration {
    /// The number of nanoseconds that the duration takes up.
    pub nanoseconds: NonZeroU64,
}

impl NonZeroDuration {
    /// One nanosecond.
    pub const NANOSECOND: NonZeroDuration =
        NonZeroDuration::from_duration(Duration::NANOSECOND).unwrap();

    /// One second.
    pub const SECOND: NonZeroDuration = NonZeroDuration::from_duration(Duration::SECOND).unwrap();

    /// Converts the duration to a [zeroable one](Duration).
    #[must_use]
    pub fn get(self) -> Duration {
        Duration {
            nanoseconds: self.nanoseconds.get(),
        }
    }

    /// Converts a duration to a non-zero one if it is not zero.
    #[must_use]
    pub const fn from_duration(duration: Duration) -> Option<NonZeroDuration> {
        match NonZeroU64::new(duration.nanoseconds) {
            Some(nanoseconds) => Some(NonZeroDuration { nanoseconds }),
            None => None,
        }
    }
}
