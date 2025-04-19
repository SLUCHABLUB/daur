mod non_zero;

pub use non_zero::NonZeroInstant;
use std::num::NonZeroU32;

use crate::Ratio;
use crate::time::{Duration, Mapping, real};
use std::ops::{Add, AddAssign, Sub};

/// An instant in musical time.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    /// The duration since the starting point
    pub since_start: Duration,
}

impl Instant {
    /// The starting point.
    pub const START: Instant = Instant {
        since_start: Duration::ZERO,
    };

    // TODO: move to its own mapping
    /// Gets the offset in samples from the staring point.
    #[must_use]
    pub fn to_sample_index(self, mapping: &Mapping, sample_rate: NonZeroU32) -> usize {
        let duration = mapping.real_time_offset(self);

        let sample = duration / real::NonZeroDuration::SECOND * Ratio::integer(sample_rate.get());

        sample.to_usize()
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

// TODO: derive
impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.since_start - rhs.since_start
    }
}
