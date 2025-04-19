//! Type pertaining to [`Audio`].

mod sample;
mod source;

pub use sample::Sample;
pub use source::Source;

use crate::time::real::Duration;
use crate::time::{Instant, Mapping, Period};
use crate::view::Context;
use hound::{Error, SampleFormat, WavReader};
use itertools::{EitherOrBoth, Itertools};
use std::cmp::max;
use std::io::Read;
use std::num::{NonZeroU32, NonZeroU128};

/// Some stereo 64-bit floating point audio.
#[doc(hidden)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Audio {
    sample_rate: NonZeroU32,
    /// The left and right channels, in that order.
    channels: [Vec<Sample>; 2],
}

impl Audio {
    /// Returns the number of stereo sample-pairs
    #[must_use]
    pub fn sample_count(&self) -> usize {
        max(self.channels[0].len(), self.channels[1].len())
    }

    // TODO: use a mapping
    /// Returns the duration of the audio.
    #[must_use]
    pub fn duration(&self) -> Duration {
        let sample_count = self.sample_count() as u128;
        let sample_rate = NonZeroU128::from(self.sample_rate);
        let nanoseconds_per_second = u128::from(Duration::SECOND.nanoseconds);

        // TODO: round
        // sample_count * samples_per_nanosecond
        #[expect(clippy::arithmetic_side_effects, reason = "we encapsulate in u128")]
        #[expect(clippy::integer_division, reason = "todo")]
        let nanoseconds = sample_count * sample_rate.get() / nanoseconds_per_second;
        let nanoseconds = u64::try_from(nanoseconds).unwrap_or(u64::MAX);

        Duration { nanoseconds }
    }

    /// An iterator of the samples converted to mono
    pub fn mono_samples(&self) -> impl Iterator<Item = Sample> + use<'_> {
        Itertools::zip_longest(self.channels[0].iter(), self.channels[1].iter()).map(|either| {
            match either {
                EitherOrBoth::Both(left, right) => (*left + *right) / 2,
                EitherOrBoth::Left(sample) | EitherOrBoth::Right(sample) => *sample,
            }
        })
    }

    /// Returns the period of the audio
    #[must_use]
    pub fn period(&self, start: Instant, mapping: &Mapping) -> Period {
        mapping.period(start, self.duration())
    }

    /// Draws an overview of the audio.
    pub fn draw_overview(
        &self,
        context: &mut dyn Context,
        full_period: Period,
        visible_period: Period,
        mapping: &Mapping,
    ) {
        // TODO: draw loudness graph
        let _ = (self, context, full_period, visible_period, mapping);
    }

    /// Returns an [audio source](rodio::Source) for the audio.
    pub fn to_source(&self, offset: usize) -> Source {
        Source::new(self.clone(), offset)
    }
}

impl<R: Read> TryFrom<WavReader<R>> for Audio {
    type Error = Error;

    fn try_from(mut reader: WavReader<R>) -> Result<Audio, Error> {
        let spec = reader.spec();
        let samples: Vec<_> = match spec.sample_format {
            SampleFormat::Float => reader
                .samples::<f32>()
                .map_ok(Sample::from_f32)
                .try_collect()?,
            SampleFormat::Int => reader
                .samples::<i32>()
                .map_ok(Sample::from_i32)
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

        let sample_rate = NonZeroU32::new(spec.sample_rate)
            .ok_or(Error::FormatError("encountered a sample rate of zero"))?;

        Ok(Audio {
            sample_rate,
            channels,
        })
    }
}
