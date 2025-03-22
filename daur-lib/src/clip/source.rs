use crate::audio;
use std::time::Duration;

/// An [audio source](rodio::Source) for a [clip](crate::Clip)
#[derive(Debug)]
#[must_use]
pub enum Source {
    /// A source from an audio clip
    Audio(audio::Source),
    // TODO: add plugins that can render the notes
    /// A source from a note clip
    Notes,
}

impl Iterator for Source {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Source::Audio(source) => source.next(),
            Source::Notes => None,
        }
    }
}

impl rodio::Source for Source {
    fn current_frame_len(&self) -> Option<usize> {
        match self {
            Source::Audio(source) => source.current_frame_len(),
            Source::Notes => None,
        }
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        match self {
            Source::Audio(source) => source.sample_rate(),
            // TODO: take from plugin?
            Source::Notes => 44_100,
        }
    }

    fn total_duration(&self) -> Option<Duration> {
        match self {
            Source::Audio(source) => source.total_duration(),
            Source::Notes => None,
        }
    }
}
