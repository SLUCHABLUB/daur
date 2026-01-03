use crate::metre;
use crate::metre::Duration;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Add;
use std::ops::Sub;

/// An instant in musical time relative to some other instant.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Instant {
    /// The duration since some defined instant.
    pub since_start: Duration,
}

impl Add<Instant> for metre::Instant {
    type Output = metre::Instant;

    fn add(self, rhs: Instant) -> metre::Instant {
        self + rhs.since_start
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.since_start - rhs.since_start
    }
}
