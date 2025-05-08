use crate::audio::Audio;
use crate::real_time;
use rodio::source::SeekError;
use std::time::Duration;

/// An [audio source](rodio::Source) for an [audio](Audio).
#[derive(Clone, Debug)]
#[must_use = "AudioSource is an iterator"]
pub struct Source {
    audio: Audio,
    right: bool,
    /// The current sample that the iterator is on, from the beginning
    sample_index: usize,
}

impl Source {
    pub(super) fn new(audio: Audio) -> Source {
        Source {
            audio,
            right: false,
            sample_index: 0,
        }
    }
}

impl Iterator for Source {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = *self.audio.samples.get(self.sample_index)?;
        let sample = if self.right { pair.right } else { pair.left };

        if self.right {
            self.sample_index = self.sample_index.saturating_add(1);
        }
        self.right = !self.right;

        Some(sample.to_f32())
    }
}

impl rodio::Source for Source {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.audio.samples.len().saturating_sub(self.sample_index))
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.audio.sample_rate.samples_per_second.get()
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from(self.audio.duration()))
    }

    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        let sample_delta =
            (real_time::Duration::from(pos) / self.audio.sample_rate.sample_duration()).to_usize();
        self.sample_index = self.sample_index.saturating_add(sample_delta);

        Ok(())
    }
}
