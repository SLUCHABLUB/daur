//! Type pertaining to [`Audio`].

mod config;
mod pair;
mod player;
mod sample;
mod sample_rate;
mod source;

pub use pair::Pair;
pub use sample::Sample;
pub use sample_rate::SampleRate;

pub(crate) use config::Config;
pub(crate) use player::Player;
pub(crate) use source::Source;

use crate::Ratio;
use crate::musical_time::{Instant, Mapping, Period};
use crate::real_time::Duration;
use crate::view::Context;
use anyhow::Result;
use hound::{SampleFormat, WavReader};
use itertools::Itertools as _;
use rubato::{FastFixedIn, PolynomialDegree, Resampler as _};
use std::borrow::Cow;
use std::cmp::max;
use std::io::Read;
use std::iter::zip;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign};

// TODO: add sample rate macro to make the test more readable
/// Some stereo 64-bit floating point audio.
///
/// # Addition
///
/// Two pieces of audio can be added together, using both `+` and `+=`.
/// When this is done, the sample rate is taken from the audio on the left.
/// See example:
///
/// ```ignore
/// let audio_one = ...;
/// let audio_two = ...;
///
/// assert_eq!(audio_one.sample_rate.samples_per_second.get(), 44_100);
/// assert_eq!(audio_two.sample_rate.samples_per_second.get(), 48_000);
///
/// let output = audio_one + audio_two;
///
/// assert_eq!(output.sample_rate.samples_per_second.get(), 44_100);
/// ```
#[doc(hidden)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Audio {
    /// The sample rate of the audio.
    pub sample_rate: SampleRate,
    /// The left and right channels, in that order.
    pub samples: Vec<Pair>,
}

impl Audio {
    /// Constructs an empty audio with the given sample rate.
    #[must_use]
    pub const fn empty(sample_rate: SampleRate) -> Audio {
        Audio {
            sample_rate,
            samples: Vec::new(),
        }
    }

    // TODO: use a mapping
    /// Returns the duration of the audio.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.sample_rate.sample_duration().get() * Ratio::from_usize(self.samples.len())
    }

    /// Resamples the audio to the given sample rate.
    #[must_use]
    pub fn resample(&self, sample_rate: SampleRate) -> Cow<Audio> {
        if self.sample_rate == sample_rate {
            return Cow::Borrowed(self);
        }

        match self.try_resample(sample_rate) {
            Ok(audio) => Cow::Owned(audio),
            // this should be unreachable
            Err(error) => {
                // TODO: log the error
                drop(error);
                Cow::Borrowed(self)
            }
        }
    }

    fn try_resample(&self, sample_rate: SampleRate) -> Result<Audio> {
        const ALL_CHANNELS_ENABLED: Option<&[bool]> = None;
        const CHANNEL_COUNT: usize = 2;
        // we want exact resampling
        const MAX_RESAMPLE_RATIO_RELATIVE: f64 = 1.0;

        let input_sample_rate = self.sample_rate.samples_per_second.get();
        let output_sample_rate = sample_rate.samples_per_second.get();

        let ratio = f64::from(output_sample_rate) / f64::from(input_sample_rate);
        let sample_count = self.samples.len();

        // TODO: allow the user to select an implementation?
        let mut resampler = FastFixedIn::new(
            ratio,
            MAX_RESAMPLE_RATIO_RELATIVE,
            PolynomialDegree::Septic,
            sample_count,
            CHANNEL_COUNT,
        )?;

        let (left, right): (Vec<_>, Vec<_>) = self
            .samples
            .iter()
            .map(|pair| (pair.left.to_f64(), pair.right.to_f64()))
            .unzip();

        let mut output = resampler.process(&[left, right], ALL_CHANNELS_ENABLED)?;

        let left = output.pop().unwrap_or_default();
        let right = output.pop().unwrap_or_default();

        let samples = zip(left, right).map(Pair::from).collect();

        Ok(Audio {
            sample_rate,
            samples,
        })
    }

    // TODO: remove
    pub(crate) fn offset(&self, offset: usize) -> Audio {
        let mut samples = vec![Pair::ZERO; offset];
        samples.extend_from_slice(&self.samples);

        Audio {
            sample_rate: self.sample_rate,
            samples,
        }
    }

    /// Returns the period of the audio
    #[must_use]
    pub(crate) fn period(&self, start: Instant, mapping: &Mapping) -> Period {
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
    pub(crate) fn into_source(self) -> Source {
        Source::new(self)
    }
}

impl<R: Read> TryFrom<WavReader<R>> for Audio {
    type Error = hound::Error;

    fn try_from(mut reader: WavReader<R>) -> hound::Result<Audio> {
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

        // TODO: use byte muck
        #[expect(
            clippy::indexing_slicing,
            clippy::missing_asserts_for_indexing,
            reason = "chunks_exact is exact"
        )]
        let samples = match spec.channels {
            1 => samples.into_iter().map(Pair::from).collect(),
            2 => samples
                .chunks_exact(2)
                .map(|chunk| Pair {
                    left: chunk[0],
                    right: chunk[1],
                })
                .collect(),
            _ => return Err(hound::Error::Unsupported),
        };

        let samples_per_second = NonZeroU32::new(spec.sample_rate).ok_or(
            hound::Error::FormatError("encountered a sample rate of zero"),
        )?;
        let sample_rate = SampleRate { samples_per_second };

        Ok(Audio {
            sample_rate,
            samples,
        })
    }
}

impl Add<&Audio> for Audio {
    type Output = Audio;

    fn add(mut self, rhs: &Audio) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<&Audio> for Audio {
    fn add_assign(&mut self, rhs: &Audio) {
        let rhs = rhs.resample(self.sample_rate);

        let sample_count = max(self.samples.len(), rhs.samples.len());
        self.samples.resize(sample_count, Pair::ZERO);

        for (lhs, rhs) in zip(&mut self.samples, &rhs.samples) {
            *lhs += *rhs;
        }
    }
}
