use crate::audio::Audio;
use rodio::Source;
use std::time::Duration;

pub struct AudioSource {
    audio: Audio,
    channel: bool,
    /// The current sample that the iterator is on, from the beginning
    sample: usize,
}

impl AudioSource {
    pub fn new(audio: Audio, sample: usize) -> AudioSource {
        AudioSource {
            audio,
            channel: false,
            sample,
        }
    }
}

impl Iterator for AudioSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = *self.audio.channels[usize::from(self.channel)].get(self.sample)?;

        if self.channel {
            self.sample += 1;
        }
        self.channel = !self.channel;

        #[allow(clippy::cast_possible_truncation)]
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
