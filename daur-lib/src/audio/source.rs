use crate::audio::sample;
use crate::{Audio, time};
use rodio::source::SeekError;
use std::time::Duration;

// TODO: use `Audio::interleaved_samples`
/// An [audio source](rodio::Source) for an [audio](Audio).
#[derive(Clone, Debug)]
#[must_use = "`Source` is an iterator"]
pub struct Source {
    audio: Audio,
    right: bool,
    position: sample::Instant,
}

impl Source {
    pub(super) fn new(audio: Audio) -> Source {
        Source {
            audio,
            right: false,
            position: sample::Instant::START,
        }
    }
}

impl Iterator for Source {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.position.since_start > self.audio.duration() {
            return None;
        }

        let [left, right] = self.audio.sample_pair(self.position);
        let sample = if self.right { right } else { left };

        if self.right {
            self.position += sample::Duration::SAMPLE;
        }
        self.right = !self.right;

        Some(sample.to_f32())
    }
}

impl rodio::Source for Source {
    fn current_frame_len(&self) -> Option<usize> {
        let remaining = self.audio.duration() - self.position.since_start;
        Some(remaining.samples)
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.audio.sample_rate.samples_per_second.get()
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from(self.audio.real_duration()))
    }

    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        let duration = time::Duration::from(pos) * self.audio.sample_rate;
        self.position += duration;

        Ok(())
    }
}
