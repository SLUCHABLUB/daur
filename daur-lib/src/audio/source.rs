use crate::audio::Audio;
use std::time::Duration;

/// An [audio source](rodio::Source) for an [audio](Audio).
#[derive(Debug)]
#[must_use = "AudioSource is an iterator"]
pub struct Source {
    audio: Audio,
    right: bool,
    /// The current sample that the iterator is on, from the beginning
    sample: usize,
}

impl Source {
    pub(super) fn new(audio: Audio, sample: usize) -> Source {
        Source {
            audio,
            right: false,
            sample,
        }
    }
}

impl Iterator for Source {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let channel = if self.right {
            &self.audio.channels[1]
        } else {
            &self.audio.channels[0]
        };
        let sample = *channel.get(self.sample)?;

        if self.right {
            self.sample = self.sample.saturating_add(1);
        }
        self.right = !self.right;

        Some(sample.to_f32())
    }
}

impl rodio::Source for Source {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.audio.sample_count().saturating_sub(self.sample))
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.audio.sample_rate.get()
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.audio.duration())
    }
}
