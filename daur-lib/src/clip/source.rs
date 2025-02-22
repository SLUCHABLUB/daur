use crate::audio::AudioSource;
use rodio::Source;
use std::time::Duration;

/// A [`Source`] for a [`Clip`](crate::Clip)
#[derive(Debug)]
#[must_use]
pub enum ClipSource {
    /// A source from an audio clip
    Audio(AudioSource),
    // TODO: add plugins that can render the notes
    /// A source from a notes clip
    Notes,
}

impl Iterator for ClipSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ClipSource::Audio(source) => source.next(),
            ClipSource::Notes => None,
        }
    }
}

impl Source for ClipSource {
    fn current_frame_len(&self) -> Option<usize> {
        match self {
            ClipSource::Audio(source) => source.current_frame_len(),
            ClipSource::Notes => None,
        }
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        match self {
            ClipSource::Audio(source) => source.sample_rate(),
            // TODO: take from plugin?
            ClipSource::Notes => 44_100,
        }
    }

    fn total_duration(&self) -> Option<Duration> {
        match self {
            ClipSource::Audio(source) => source.total_duration(),
            ClipSource::Notes => None,
        }
    }
}
