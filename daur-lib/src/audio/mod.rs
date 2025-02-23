mod source;

pub use source::AudioSource;

use crate::time::{Instant, Mapping, Period};
use hound::{Error, SampleFormat, WavReader};
use itertools::{EitherOrBoth, Itertools};
use num::{rational, Integer as _};
use std::io::Read;
use std::num::FpCategory;
use std::time::Duration;

/// Some stereo 64-bit floating point audio
#[derive(Clone, PartialEq, Debug)]
pub struct Audio {
    sample_rate: u32,
    // On the interval [-1; 1]
    // TODO: use non nan type and derive Eq
    channels: [Vec<f64>; 2],
}

impl Audio {
    /// Returns the number of stereo sample-pairs
    #[must_use]
    pub fn sample_count(&self) -> usize {
        usize::max(self.channels[0].len(), self.channels[1].len())
    }

    /// Returns the audio's duration
    #[must_use]
    pub fn duration(&self) -> Duration {
        const NANOS_PER_SEC: u64 = 1_000_000_000;

        let sample_count = self.sample_count() as u64;
        let sample_rate = u64::from(self.sample_rate);
        let nano_sample_rate = rational::Ratio::new(sample_rate, NANOS_PER_SEC);

        let (seconds, remainder) = sample_count.div_rem(&sample_rate);

        #[expect(
            clippy::arithmetic_side_effects,
            reason = "remainder and NANOS_PER_SEC fit in u32 => product fits in u64"
        )]
        let nanos = rational::Ratio::from(remainder) / nano_sample_rate;
        let nanos = nanos.round().to_integer();
        #[expect(
            clippy::cast_possible_truncation,
            reason = "remainder / sample_rate < 1 => it * NANOS_PER_SEC < NANOS_PER_SEC"
        )]
        let nanos = nanos as u32;

        Duration::new(seconds, nanos)
    }

    /// An iterator of the samples converted to mono
    pub fn mono_samples(&self) -> impl Iterator<Item = f64> + use<'_> {
        Itertools::zip_longest(self.channels[0].iter(), self.channels[1].iter()).map(|either| {
            match either {
                EitherOrBoth::Both(left, right) => (left + right) / 2.0,
                EitherOrBoth::Left(sample) | EitherOrBoth::Right(sample) => *sample,
            }
        })
    }

    /// Returns the period of the audio
    #[must_use]
    pub fn period(&self, start: Instant, mapping: &Mapping) -> Period {
        mapping.period(start, self.duration())
    }

    /// Returns a [`Source`](rodio::source::Source) for the audio
    pub fn to_source(&self, offset: usize) -> AudioSource {
        AudioSource::new(self.clone(), offset)
    }
}

impl Eq for Audio {}

// TODO: test
/// Losslessly convert an i32 sample to a f64 sample
fn int_to_float_sample(sample: i32) -> f64 {
    f64::from(sample) / (f64::from(i32::MAX) + 1.0)
}

fn clamp_float_sample(sample: f32) -> f64 {
    let sample = f64::from(sample);
    match sample.classify() {
        FpCategory::Nan => 0.0,
        FpCategory::Infinite | FpCategory::Zero | FpCategory::Subnormal | FpCategory::Normal => {
            sample.clamp(-1.0, 1.0)
        }
    }
}

impl<R: Read> TryFrom<WavReader<R>> for Audio {
    type Error = Error;

    fn try_from(mut reader: WavReader<R>) -> Result<Self, Self::Error> {
        let spec = reader.spec();
        let samples: Vec<f64> = match spec.sample_format {
            SampleFormat::Float => reader
                .samples::<f32>()
                .map_ok(clamp_float_sample)
                .try_collect()?,
            SampleFormat::Int => reader
                .samples::<i32>()
                .map_ok(int_to_float_sample)
                .try_collect()?,
        };

        #[expect(
            clippy::indexing_slicing,
            clippy::missing_asserts_for_indexing,
            reason = "chunks_exact is exact"
        )]
        let channels = match spec.channels {
            1 => [samples.clone(), samples],
            2 => samples
                .chunks_exact(2)
                .map(|chunk| (chunk[0], chunk[1]))
                .unzip()
                .into(),
            _ => return Err(Error::Unsupported),
        };

        Ok(Audio {
            sample_rate: spec.sample_rate,
            channels,
        })
    }
}
