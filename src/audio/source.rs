use crate::audio::Audio;
use rodio::Source;
use std::time::Duration;

pub struct AudioSource {
    audio: Audio,
    right: bool,
    /// The current sample that the iterator is on, from the beginning
    sample: usize,
}

impl AudioSource {
    pub fn new(audio: Audio, sample: usize) -> AudioSource {
        AudioSource {
            audio,
            right: false,
            sample,
        }
    }
}

impl Iterator for AudioSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let channel = if self.right {
            &self.audio.channels[0]
        } else {
            &self.audio.channels[1]
        };
        let sample = *channel.get(self.sample)?;

        if self.right {
            self.sample = self.sample.saturating_add(1);
        }
        self.right = !self.right;

        #[expect(
            clippy::cast_possible_truncation,
            reason = "playback audio doesn't support f64 precision"
        )]
        Some(sample as f32)
    }
}

impl Source for AudioSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.audio.sample_count().saturating_sub(self.sample))
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.audio.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.audio.duration())
    }
}
