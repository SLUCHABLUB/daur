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

use crate::audio::sample::ZeroRateError;
use crate::time;
use anyhow::Result;
use bytemuck::cast_slice;
use getset::CopyGetters;
use log::error;
use rubato::{FastFixedIn, PolynomialDegree, Resampler as _};
use std::borrow::Cow;
use std::cmp::max;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};
use thiserror::Error;

/// Some stereo 64-bit floating point audio.
#[cfg_attr(doc, doc(hidden))]
#[derive(Clone, Eq, PartialEq, Debug, CopyGetters)]
pub struct Audio<'samples> {
    /// The sample rate of the audio.
    #[get_copy = "pub"]
    sample_rate: sample::Rate,
    /// The left and right channels, in that order.
    channels: [Cow<'samples, [Sample]>; 2],
}

impl Audio<'_> {
    /// Constructs an empty audio with the given sample rate.
    #[must_use]
    pub const fn empty(sample_rate: sample::Rate) -> Self {
        Audio {
            sample_rate,
            channels: [Cow::Borrowed(&[]), Cow::Borrowed(&[])],
        }
    }

    /// Constructs an empty audio with the given sample rate and capacity.
    #[must_use]
    pub fn with_capacity(sample_rate: sample::Rate, capacity: sample::Duration) -> Self {
        Audio {
            sample_rate,
            channels: [
                Cow::Owned(Vec::with_capacity(capacity.samples)),
                Cow::Owned(Vec::with_capacity(capacity.samples)),
            ],
        }
    }

    /// Constructs a reference to the audio.
    #[must_use]
    pub fn as_ref(&self) -> Audio {
        Audio {
            sample_rate: self.sample_rate,
            channels: [
                Cow::Borrowed(&self.channels[0]),
                Cow::Borrowed(&self.channels[1]),
            ],
        }
    }

    /// Constructs an owned audio.
    #[must_use]
    pub fn into_owned(self) -> Audio<'static> {
        let [left, right] = self.channels;

        Audio {
            sample_rate: self.sample_rate,
            channels: [
                Cow::Owned(left.into_owned()),
                Cow::Owned(right.into_owned()),
            ],
        }
    }

    pub(crate) fn extend_to(&mut self, duration: sample::Duration) {
        for channel in &mut self.channels {
            if channel.len() < duration.samples {
                channel.to_mut().resize(duration.samples, Sample::ZERO);
            }
        }
    }

    pub(crate) fn truncate_silence(&mut self, minimum_duration: sample::Duration) {
        for channel in &mut self.channels {
            let Some(extra) = channel
                .iter()
                .skip(minimum_duration.samples)
                .position(|sample| *sample == Sample::ZERO)
            else {
                continue;
            };

            let new_len = minimum_duration.samples.saturating_add(extra);

            channel.to_mut().truncate(new_len);
        }
    }

    /// Returns the sample-time duration of the audio.
    #[must_use]
    pub fn duration(&self) -> sample::Duration {
        sample::Duration {
            samples: max(self.channels[0].len(), self.channels[1].len()),
        }
    }

    /// Returns the real-time duration of the audio.
    #[must_use]
    pub fn real_duration(&self) -> time::Duration {
        self.duration() / self.sample_rate
    }

    /// Resamples the audio to the given sample rate.
    #[must_use]
    pub fn resample(&self, sample_rate: sample::Rate) -> Audio {
        if self.sample_rate == sample_rate {
            return self.as_ref();
        }

        match self.try_resample(sample_rate) {
            Ok(audio) => audio,
            // this should be unreachable
            Err(error) => {
                error!("Error when trying to resample audio: {error} ({error:?})");
                self.as_ref()
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
        let sample_count = self.duration().samples;

        // TODO: allow the user to select an implementation?
        let mut resampler = FastFixedIn::new(
            ratio,
            MAX_RESAMPLE_RATIO_RELATIVE,
            PolynomialDegree::Septic,
            sample_count,
            CHANNEL_COUNT,
        )?;

        let [left, right] = &self.channels;

        let mut output =
            resampler.process(&[cast_slice(left), cast_slice(right)], ALL_CHANNELS_ENABLED)?;

        let left = output
            .pop()
            .unwrap_or_default()
            .into_iter()
            .map(Sample::new)
            .collect();
        let right = output
            .pop()
            .unwrap_or_default()
            .into_iter()
            .map(Sample::new)
            .collect();

        Ok(Audio {
            sample_rate,
            channels: [left, right],
        })
    }

    pub(crate) fn superpose(&mut self, other: &Audio) {
        self.superpose_with_offset(other, sample::Duration::ZERO);
    }

    pub(crate) fn superpose_with_offset(&mut self, other: &Audio, offset: sample::Duration) {
        let other = other.resample(self.sample_rate);

        for index in 0..other.duration().samples {
            let instant_in_other = sample::Instant::from_index(index);
            let instant_in_self = instant_in_other + offset;

            let [self_left, self_right] = self.sample_pair_mut(instant_in_self);
            let [other_left, other_right] = other.sample_pair(instant_in_other);

            *self_left += other_left;
            *self_right += other_right;
        }
    }

    pub(crate) fn read_from_file<P: AsRef<Path>>(
        file: P,
    ) -> Result<Audio<'static>, ImportAudioError> {
        let extension = file.as_ref().extension().and_then(OsStr::to_str);

        let file = File::open(file.as_ref())?;

        let stream = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

        let probe = get_probe();

        let mut hint = Hint::new();

        if let Some(extension) = extension {
            hint.with_extension(extension);
        }

        let mut format = probe
            .format(
                &hint,
                stream,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            )?
            .format;

        let no_tracks = || ImportAudioError::Symphonia(SymphoniaError::DecodeError("no tracks"));

        let mut track = format.default_track().ok_or(no_tracks())?;

        let codecs = get_codecs();
        let decoder_options = DecoderOptions { verify: true };

        let mut decoder = codecs.make(&track.codec_params, &decoder_options)?;

        let mut sample_rate = None;

        let mut left_channel = Vec::new();
        let mut right_channel = Vec::new();

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(SymphoniaError::ResetRequired) => {
                    track = format.default_track().ok_or(no_tracks())?;
                    decoder = codecs.make(&track.codec_params, &decoder_options)?;
                    continue;
                }
                Err(SymphoniaError::IoError(error)) if error.kind() == ErrorKind::UnexpectedEof => {
                    break;
                }
                error => error?,
            };

            let audio_ref = decoder.decode(&packet)?;

            let rate = sample::Rate::try_from(audio_ref.spec().rate)?;

            match sample_rate {
                None => sample_rate = Some(rate),
                Some(sample_rate) => {
                    if sample_rate != rate {
                        return Err(ImportAudioError::SampleRateInconsistency);
                    }
                }
            }

            let mut audio_ref_32 = audio_ref.make_equivalent();
            audio_ref.convert::<f32>(&mut audio_ref_32);

            let audio_ref = audio_ref_32;

            let planes = audio_ref.planes();
            let planes = planes.planes();

            let empty: &[f32] = &[];

            let [left, right] = match planes {
                [] => [empty, empty],
                [mono] => [*mono, *mono],
                [left, right, ..] => [*left, *right],
            };

            for sample in left {
                left_channel.push(Sample::new(*sample));
            }

            for sample in right {
                right_channel.push(Sample::new(*sample));
            }
        }

        Ok(Audio {
            sample_rate: sample_rate.ok_or(ImportAudioError::NoPackets)?,
            channels: [Cow::Owned(left_channel), Cow::Owned(right_channel)],
        })
    }

    /// Returns a subsection of the audio.
    #[must_use]
    pub fn subsection(&self, period: sample::Period) -> Audio {
        Audio {
            sample_rate: self.sample_rate,
            channels: [
                Cow::Borrowed(
                    self.channels[0]
                        .get(period.range())
                        .or_else(|| self.channels[0].get(period.start.index()..))
                        .unwrap_or(&[]),
                ),
                Cow::Borrowed(
                    self.channels[1]
                        .get(period.range())
                        .or_else(|| self.channels[0].get(period.start.index()..))
                        .unwrap_or(&[]),
                ),
            ],
        }
    }

    /// Returns a left-right sample pair.
    #[must_use]
    pub fn sample_pair(&self, instant: sample::Instant) -> [Sample; 2] {
        [
            self.channels[0]
                .get(instant.index())
                .copied()
                .unwrap_or(Sample::ZERO),
            self.channels[1]
                .get(instant.index())
                .copied()
                .unwrap_or(Sample::ZERO),
        ]
    }

    /// Returns a mutable reference to a left-right sample pair.
    #[must_use]
    pub fn sample_pair_mut(&mut self, instant: sample::Instant) -> [&mut Sample; 2] {
        self.extend_to(instant.since_start + sample::Duration::SAMPLE);

        let [left, right] = &mut self.channels;

        let left = left.to_mut();
        let right = right.to_mut();

        #[expect(clippy::indexing_slicing, reason = "we resize the vectors first")]
        [&mut left[instant.index()], &mut right[instant.index()]]
    }
}

/// An error when importing audio from a file.
#[derive(Debug, Error)]
pub enum ImportAudioError {
    /// An error when reading the file.
    #[error("Error reading the audio file: {0}")]
    Io(#[from] io::Error),
    /// AN error when decoding the file.
    #[error("{0}")]
    Symphonia(#[from] SymphoniaError),
    /// The file had a sample rate of 0.
    #[error("{0}")]
    ZeroSampleRate(#[from] ZeroRateError),
    /// The packets in the file had different sample rates.
    #[error("audio packets had different sample rates")]
    SampleRateInconsistency,
    /// The audio file contained no audio packets.
    #[error("no audio packets")]
    NoPackets,
}
