mod non_zero;

pub use non_zero::NonZeroInstant;
use std::num::{NonZeroU32, NonZeroU128};

use crate::time::{Duration, Mapping};
use num::Integer as _;
use saturating_cast::SaturatingCast as _;
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
    /// Gets the offset in samples from the staring point
    #[must_use]
    pub fn to_sample(self, mapping: &Mapping, sample_rate: NonZeroU32) -> usize {
        const NANOS_PER_SECOND: u128 = 1_000_000_000;
        const HALF: u128 = 500_000_000;

        let sample_rate = NonZeroU128::from(sample_rate);
        let duration = mapping.real_time_offset(self);

        // < 2^64 * 10^9 < 2^94
        let nanos = duration.as_nanos();

        // * 2 since we are always in stereo
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "nanos < 2^94, sample_rate < 2^32 => nano_sample < 2^(94 + 32 + 1) < 2^128"
        )]
        let nano_sample = nanos * sample_rate.get() * 2;

        let (mut sample, remainder) = nano_sample.div_rem(&NANOS_PER_SECOND);

        if remainder > HALF {
            sample = sample.saturating_add(1);
        }

        sample.saturating_cast()
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
