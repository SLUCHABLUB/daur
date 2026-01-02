use crate::audio::Sample;
use crate::audio::sample;
use crate::audio::sample::Duration;
use crate::audio::sample::Instant;
use std::cmp::max;

/// A section of an [audio](super::Audio).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Subsection<'audio> {
    /// The sample rate of the audio.
    pub sample_rate: sample::Rate,
    /// The left and right channels of the audio, in said order.
    pub channels: [&'audio [Sample]; 2],
}

impl Subsection<'_> {
    /// Returns the duration of the audio.
    #[must_use]
    pub fn duration(&self) -> Duration {
        Duration {
            samples: max(self.channels[0].len(), self.channels[1].len()),
        }
    }

    // TODO: take a relative instant
    /// Returns the left-right sample pair at the given position.
    #[must_use]
    pub fn sample_pair(&self, position: Instant) -> [Sample; 2] {
        self.channels.map(|channel| {
            channel
                .get(position.index())
                .copied()
                .unwrap_or(Sample::ZERO)
        })
    }
}
