mod non_zero;

pub use non_zero::NonZeroInstant;

use crate::metre::{Duration, relative};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// An instant in musical time.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    /// The duration since the starting point.
    pub since_start: Duration,
}

impl Instant {
    /// The starting point.
    pub const START: Instant = Instant {
        since_start: Duration::ZERO,
    };

    pub(crate) fn relative_to(self, other: Instant) -> relative::Instant {
        relative::Instant {
            since_start: self - other,
        }
    }
}

// TODO: derive
impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(mut self, rhs: Duration) -> Instant {
        self += rhs;
        self
    }
}

// TODO: derive
impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.since_start += rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(mut self, rhs: Duration) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.since_start -= rhs;
    }
}

// TODO: derive
impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.since_start - rhs.since_start
    }
}
