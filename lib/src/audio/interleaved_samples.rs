//! Items pertaining to `InterleavedSamples`.

use crate::Audio;
use crate::audio::Sample;
use crate::audio::sample;
use crate::audio::sample::Duration;
use crate::audio::sample::Instant;
use std::borrow::Cow;
use std::iter::FusedIterator;

/// An iterator over samples in an [audio clipp](Audio) in interleaved format.
///
/// The first sample returned is from the left channel.
#[derive(Clone, Eq, PartialEq, Debug)]
#[must_use]
pub struct InterleavedSamples<'audio> {
    /// The audio being iterated over.
    audio: Cow<'audio, Audio>,
    /// How far in the audio the iterator has iterated.
    position: Instant,
    /// Whether the next sample to return should be from the right channel.
    right_channel: bool,
}

impl InterleavedSamples<'_> {
    /// Returns the sample-rate of the audio from which the samples come.
    #[must_use]
    pub fn rate(&self) -> sample::Rate {
        self.audio.sample_rate
    }

    /// Skip forward in the audio by a given amount of time.
    pub(super) fn skip_forward(&mut self, duration: Duration) {
        self.position += duration;
    }
}

impl Iterator for InterleavedSamples<'_> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.since_start > self.audio.duration() {
            return None;
        }

        let [left, right] = self.audio.sample_pair(self.position);
        let sample = if self.right_channel { right } else { left };

        if self.right_channel {
            self.position += Duration::SAMPLE;
        }
        self.right_channel = !self.right_channel;

        Some(sample)
    }
}

impl FusedIterator for InterleavedSamples<'_> {}

impl Audio {
    /// Returns an iterator over the samples in interleaved format.
    pub fn interleaved_samples(&self) -> InterleavedSamples<'_> {
        InterleavedSamples {
            audio: Cow::Borrowed(self),
            position: Instant::START,
            right_channel: false,
        }
    }

    /// Returns an iterator over the samples in interleaved format.
    pub fn into_interleaved_samples(self) -> InterleavedSamples<'static> {
        InterleavedSamples {
            audio: Cow::Owned(self),
            position: Instant::START,
            right_channel: false,
        }
    }
}
