//! Type pertaining to [`Audio`].

pub mod sample;

mod config;
mod fixed_length;
mod import;
mod interleaved_samples;
mod player;
mod resample;
mod source;
mod subsection;

pub use fixed_length::FixedLength;
pub use import::ImportError;
pub use interleaved_samples::InterleavedSamples;
#[doc(inline)]
pub use sample::Sample;
pub use subsection::Subsection;

pub(crate) use config::Config;
pub(crate) use player::Player;
pub(crate) use source::Source;

use crate::time;
use hound::SampleFormat;
use hound::WavSpec;
use hound::WavWriter;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::max;
use std::path::Path;

/// Some stereo 64-bit floating point audio.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Audio {
    /// The sample rate of the audio.
    pub sample_rate: sample::Rate,
    /// The left and right channels of the audio, in said order.
    pub channels: [Vec<Sample>; 2],
}

impl Audio {
    /// Constructs an empty audio with the given sample rate.
    #[must_use]
    pub const fn empty(sample_rate: sample::Rate) -> Self {
        Audio {
            sample_rate,
            channels: [Vec::new(), Vec::new()],
        }
    }

    /// Constructs an empty audio with the given sample rate and capacity.
    #[must_use]
    pub fn with_capacity(sample_rate: sample::Rate, capacity: sample::Duration) -> Self {
        Audio {
            sample_rate,
            channels: [
                Vec::with_capacity(capacity.samples),
                Vec::with_capacity(capacity.samples),
            ],
        }
    }

    /// Extend the audio clip with silence to fit *at least* the given duration.
    pub(crate) fn extend_to(&mut self, duration: sample::Duration) {
        for channel in &mut self.channels {
            if channel.len() < duration.samples {
                channel.resize(duration.samples, Sample::ZERO);
            }
        }
    }

    /// Remove silence at the end of the clip.
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

            channel.truncate(new_len);
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

    /// Superposes another audio clip onto this audio clip.
    pub(crate) fn superpose(&mut self, other: &Audio) {
        self.superpose_with_offset(other, sample::Duration::ZERO);
    }

    /// Superposes another audio clip (offset by an offset) onto this audio clip.
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

    /// Returns a subsection of the audio.
    #[must_use]
    pub fn subsection(&self, period: sample::Period) -> Subsection<'_> {
        Subsection {
            sample_rate: self.sample_rate,
            channels: [
                self.channels[0]
                    .get(period.range())
                    .or_else(|| self.channels[0].get(period.start.index()..))
                    .unwrap_or(&[]),
                self.channels[1]
                    .get(period.range())
                    .or_else(|| self.channels[0].get(period.start.index()..))
                    .unwrap_or(&[]),
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

        #[expect(clippy::indexing_slicing, reason = "we resize the vectors first")]
        [&mut left[instant.index()], &mut right[instant.index()]]
    }

    /// Exports the clip to a file at the given path.
    pub(crate) fn export(&self, to: &Path) -> Result<(), hound::Error> {
        let spec = WavSpec {
            channels: 2,
            sample_rate: self.sample_rate.samples_per_second.get(),
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        let mut writer = WavWriter::create(to, spec)?;

        for sample in self.interleaved_samples() {
            writer.write_sample(sample.to_f32())?;
        }

        Ok(())
    }
}
