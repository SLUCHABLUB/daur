use crate::Ratio;
use crate::time::{Duration, NonZeroDuration};
use rodio::cpal;
use std::num::{NonZeroU32, NonZeroU64};
use thiserror::Error;

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

/// An error raised when a sample rate of zero is attempted to be constructed.
#[derive(Copy, Clone, Debug, Error)]
#[error("sample rates cannot be zero")]
pub struct ZeroSampleRateError;

impl TryFrom<cpal::SampleRate> for SampleRate {
    type Error = ZeroSampleRateError;

    fn try_from(
        cpal::SampleRate(sample_rate): cpal::SampleRate,
    ) -> Result<SampleRate, ZeroSampleRateError> {
        let samples_per_second = NonZeroU32::new(sample_rate).ok_or(ZeroSampleRateError)?;
        Ok(SampleRate { samples_per_second })
    }
}
