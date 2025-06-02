//! Type pertaining to [`Audio`].

pub mod sample;

mod config;
mod fixed_length;
mod player;
mod source;

pub use fixed_length::FixedLength;
#[doc(inline)]
pub use sample::Sample;

pub(crate) use config::Config;
pub(crate) use player::Player;
pub(crate) use source::Source;

use crate::{Ratio, time};
use anyhow::Result;
use hound::{SampleFormat, WavReader};
use itertools::Itertools as _;
use log::error;
use rubato::{FastFixedIn, PolynomialDegree, Resampler as _};
use std::borrow::Cow;
use std::cmp::max;
use std::ffi::OsStr;
use std::io;
use std::io::Read;
use std::iter::zip;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign};
use std::path::Path;
use thiserror::Error;

/// Some stereo 64-bit floating point audio.
///
/// # Addition
///
/// Two pieces of audio can be added together, using both `+` and `+=`.
/// When this is done, the sample rate is taken from the audio on the left.
/// See example:
///
/// ```
/// # use daur::{sample_rate, Audio};
///
/// # let audio_one = Audio::empty(sample_rate!(44_100 Hz));
/// # let audio_two = &Audio::empty(sample_rate!(48_000 Hz));
///
/// assert_eq!(audio_one.sample_rate, sample_rate!(44_100 Hz));
/// assert_eq!(audio_two.sample_rate, sample_rate!(48_000 Hz));
///
/// let output = audio_one + audio_two;
///
/// assert_eq!(output.sample_rate, sample_rate!(44_100 Hz));
/// ```
#[cfg_attr(doc, doc(hidden))]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Audio {
    /// The sample rate of the audio.
    pub sample_rate: sample::Rate,
    /// The left and right channels, in that order.
    pub samples: Vec<sample::Pair>,
}

/// An error that occurred whilst importing an audio file.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ImportAudioError {
    /// The file has no extension. Thus, the format cannot be inferred.
    #[error("the file has no extension; unable to infer a file type")]
    NoExtension,
    /// An io error occurred whilst reading the file.
    #[error("error when reading file: {0}")]
    ReadFile(#[from] io::Error),
    /// A wav processing error.
    #[error("{0}")]
    Hound(#[from] hound::Error),
    /// An unknown audio format was encountered.
    #[error("the `{}` audio format is not (yet) supported", _0.display())]
    UnsupportedFormat(Box<OsStr>),
}

impl Audio {
    /// Constructs an empty audio with the given sample rate.
    #[must_use]
    pub const fn empty(sample_rate: sample::Rate) -> Audio {
        Audio {
            sample_rate,
            samples: Vec::new(),
        }
    }

    /// Returns the sample-time duration of the audio.
    #[must_use]
    pub fn duration(&self) -> sample::Duration {
        sample::Duration {
            samples: self.samples.len(),
        }
    }

    /// Returns the real-time duration of the audio.
    #[must_use]
    pub fn real_duration(&self) -> time::Duration {
        self.sample_rate.sample_duration().get() * Ratio::from_usize(self.samples.len())
    }

    /// Resamples the audio to the given sample rate.
    #[must_use]
    pub fn resample(&self, sample_rate: sample::Rate) -> Cow<Audio> {
        if self.sample_rate == sample_rate {
            return Cow::Borrowed(self);
        }

        match self.try_resample(sample_rate) {
            Ok(audio) => Cow::Owned(audio),
            // this should be unreachable
            Err(error) => {
                error!("Error when trying to resample audio: {error} ({error:?})");
                Cow::Borrowed(self)
            }
        }
    }

    fn try_resample(&self, sample_rate: sample::Rate) -> Result<Audio> {
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

        let left = output
            .pop()
            .unwrap_or_default()
            .into_iter()
            .map(Sample::new);
        let right = output
            .pop()
            .unwrap_or_default()
            .into_iter()
            .map(Sample::new);

        let samples = zip(left, right)
            .map_into::<[_; 2]>()
            .map(sample::Pair::from)
            .collect();

        Ok(Audio {
            sample_rate,
            samples,
        })
    }

    pub(crate) fn add_assign_at(&mut self, other: &Audio, offset: sample::Duration) {
        let other = other.resample(self.sample_rate);

        let sample_count = max(
            self.samples.len(),
            other.samples.len().saturating_add(offset.samples),
        );
        self.samples.resize(sample_count, sample::Pair::ZERO);

        for (lhs, rhs) in zip(self.samples.iter_mut().skip(offset.samples), &other.samples) {
            *lhs += *rhs;
        }
    }

    pub(crate) fn read_from_file<P: AsRef<Path>>(file: P) -> Result<Audio, ImportAudioError> {
        let extension = file
            .as_ref()
            .extension()
            .ok_or(ImportAudioError::NoExtension)?;

        // TODO: look at the symphonia crate
        match extension.to_string_lossy().as_ref() {
            "wav" | "wave" => {
                let reader = WavReader::open(file)?;
                Ok(Audio::try_from(reader)?)
            }
            _ => Err(ImportAudioError::UnsupportedFormat(Box::from(extension))),
        }
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

        let samples = match spec.channels {
            1 => samples.into_iter().map(sample::Pair::from).collect(),
            2 => samples
                .into_iter()
                .tuples::<(_, _)>()
                .map_into::<[_; 2]>()
                .map(sample::Pair::from)
                .collect(),
            _ => return Err(hound::Error::Unsupported),
        };

        let samples_per_second = NonZeroU32::new(spec.sample_rate).ok_or(
            hound::Error::FormatError("encountered a sample rate of zero"),
        )?;
        let sample_rate = sample::Rate { samples_per_second };

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
        self.add_assign_at(rhs, sample::Duration::ZERO);
    }
}
