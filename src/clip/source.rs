use crate::audio::source::AudioSource;
use rodio::Source;
use std::time::Duration;

pub enum ClipSource {
    Audio(AudioSource),
}

impl Iterator for ClipSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ClipSource::Audio(source) => source.next(),
        }
    }
}

impl Source for ClipSource {
    fn current_frame_len(&self) -> Option<usize> {
        match self {
            ClipSource::Audio(source) => source.current_frame_len(),
        }
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        match self {
            ClipSource::Audio(source) => source.sample_rate(),
        }
    }

    fn total_duration(&self) -> Option<Duration> {
        match self {
            ClipSource::Audio(source) => source.total_duration(),
        }
    }
}
