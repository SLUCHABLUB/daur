mod source;

pub use source::AudioSource;
use std::cmp::max;

use crate::time::{Instant, Mapping, Period};
use crate::ui::{Length, NonZeroLength, Point};
use crate::view::Context;
use crate::{Colour, Ratio};
use hound::{Error, SampleFormat, WavReader};
use itertools::{EitherOrBoth, Itertools};
use num::{Integer as _, rational};
use saturating_cast::SaturatingCast as _;
use std::io::Read;
use std::num::{FpCategory, NonZeroU16, NonZeroU32};
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
        max(self.channels[0].len(), self.channels[1].len())
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

    /// Draws an overview of the audio.
    pub fn draw_overview(
        &self,
        context: &mut dyn Context,
        full_period: Period,
        visible_period: Period,
        mapping: &Mapping,
    ) {
        #![expect(
            clippy::cast_sign_loss,
            reason = "we are working with durations (unsigned)"
        )]
        #![expect(
            clippy::cast_possible_truncation,
            reason = "saturating is fine when counting samples"
        )]

        let sample_rate = f64::from(self.sample_rate);

        let left_cutoff = Period::from_endpoints(full_period.start, visible_period.start)
            .map_or(Duration::ZERO, |period| mapping.real_time_duration(period));
        let skipped_samples = sample_rate * left_cutoff.as_secs_f64();
        let skipped_samples = skipped_samples.round() as usize;

        let visible_samples =
            sample_rate * mapping.real_time_duration(visible_period).as_secs_f64();
        let visible_samples = visible_samples.round() as usize;

        let samples = self
            .mono_samples()
            .skip(skipped_samples)
            .take(visible_samples.saturating_cast());

        let Some(visible_samples) = NonZeroU32::new(visible_samples as u32) else {
            return;
        };

        let context_size = context.size();

        let Some(context_width) = NonZeroU16::new(context_size.width.inner()) else {
            return;
        };

        let sample_length = context_size.width / visible_samples;

        if sample_length == Length::ZERO {
            // since the numerator isn't zero this resulted from a rounding

            let Some(epsilon) = NonZeroLength::X_MINIMUM else {
                // TODO: we have a really small context or really many samples
                return;
            };

            let samples_per_epsilon = visible_samples.get() / NonZeroU32::from(context_width);

            samples
                .chunks(samples_per_epsilon.saturating_cast())
                .into_iter()
                .enumerate()
                .for_each(|(x, samples)| {
                    let x = epsilon.get() * x.saturating_cast::<u32>();

                    let average = samples.sum::<f64>() / f64::from(samples_per_epsilon);
                    let ratio = Ratio::approximate(-0.5 * (average - 1.0));

                    let y = context_size.height * ratio;

                    context.draw_point(Point { x, y }, Colour::WHITE);
                });
        } else {
            // TODO: float lengths
        }
    }

    /// Returns a [`Source`](source::Source) for the audio
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
