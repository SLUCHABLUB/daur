mod non_zero;

pub use non_zero::NonZeroInstant;

use crate::audio::sample;
use crate::time::Duration;
use std::ops::{Add, AddAssign, Mul, Sub};

/// An instant in real time.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    /// The duration since the compositions start.
    pub since_start: Duration,
}

impl Instant {
    /// The starting point.
    pub const START: Instant = Instant {
        since_start: Duration::ZERO,
    };
}

// TODO: derive
impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(mut self, rhs: Duration) -> Instant {
        self += rhs;
        self
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.since_start += rhs;
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Duration {
        self.since_start - rhs.since_start
    }
}

impl Mul<sample::Rate> for Instant {
    type Output = sample::Instant;

    fn mul(self, rhs: sample::Rate) -> sample::Instant {
        sample::Instant {
            since_start: self.since_start * rhs,
        }
    }
}
