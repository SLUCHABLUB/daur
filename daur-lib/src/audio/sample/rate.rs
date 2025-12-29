use crate::Ratio;
use crate::time::Duration;
use crate::time::NonZeroDuration;
use rodio::cpal;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use thiserror::Error;

/// A sample rate.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Rate {
    /// The number of samples that fit in one second.
    pub samples_per_second: NonZeroU32,
}

impl Rate {
    /// The duration of one sample.
    #[must_use]
    pub fn sample_duration(self) -> NonZeroDuration {
        let seconds_per_sample = Ratio::reciprocal_of(NonZeroU64::from(self.samples_per_second));

        let duration = Duration::SECOND * seconds_per_sample;

        NonZeroDuration::from_duration(duration).unwrap_or(NonZeroDuration::NANOSECOND)
    }

    /// Returns self in Hz as an [`f32`].
    #[must_use]
    pub fn hz(self) -> f32 {
        #![expect(
            clippy::cast_precision_loss,
            reason = "sample rates will in practice not need that high precision"
        )]
        self.samples_per_second.get() as f32
    }
}

/// An error raised when a sample rate of zero is attempted to be constructed.
#[derive(Copy, Clone, Debug, Error)]
#[error("sample rates cannot be zero")]
pub struct ZeroRateError;

impl TryFrom<cpal::SampleRate> for Rate {
    type Error = ZeroRateError;

    fn try_from(cpal::SampleRate(sample_rate): cpal::SampleRate) -> Result<Rate, ZeroRateError> {
        Rate::try_from(sample_rate)
    }
}

impl TryFrom<u32> for Rate {
    type Error = ZeroRateError;

    fn try_from(sample_rate: u32) -> Result<Rate, ZeroRateError> {
        let samples_per_second = NonZeroU32::new(sample_rate).ok_or(ZeroRateError)?;
        Ok(Rate { samples_per_second })
    }
}
