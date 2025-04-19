use crate::Ratio;
use crate::time::real::{Duration, NonZeroDuration};
use std::num::{NonZeroU32, NonZeroU64};

/// A sample rate.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SampleRate {
    /// The number of samples that fit in one second.
    pub samples_per_second: NonZeroU32,
}

impl SampleRate {
    /// The duration of one sample.
    #[must_use]
    pub fn sample_duration(self) -> NonZeroDuration {
        let seconds_per_sample = Ratio::reciprocal_of(NonZeroU64::from(self.samples_per_second));

        let duration = Duration::SECOND * seconds_per_sample;

        NonZeroDuration::from_duration(duration).unwrap_or(NonZeroDuration::NANOSECOND)
    }
}
