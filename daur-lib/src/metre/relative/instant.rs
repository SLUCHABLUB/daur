use crate::metre;
use crate::metre::Duration;
use std::ops::Add;

/// An instant in musical time relative to some other instant.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    /// The duration since some defined instant.
    pub since_start: Duration,
}

impl Add<Instant> for metre::Instant {
    type Output = metre::Instant;

    fn add(self, rhs: Instant) -> Self::Output {
        self + rhs.since_start
    }
}
