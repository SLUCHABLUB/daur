use crate::clip::source::ClipSource;
use rodio::Source;
use std::collections::VecDeque;
use std::time::Duration;

pub struct TrackSource {
    sample_rate: u32,
    sample: usize,
    clips: VecDeque<(usize, ClipSource)>,
}

impl TrackSource {
    pub fn new(
        sample_rate: u32,
        clips: VecDeque<(usize, ClipSource)>,
        sample: usize,
    ) -> TrackSource {
        TrackSource {
            sample_rate,
            sample,
            clips,
        }
    }
}

impl Iterator for TrackSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((next, _)) = self.clips.get(1) {
            if *next <= self.sample {
                self.clips.pop_front();
                return self.next();
            }
        }

        let (start, clip) = self.clips.front_mut()?;

        let started = *start <= self.sample;

        self.sample += 1;

        Some(if started {
            clip.next().unwrap_or(0.0)
        } else {
            0.0
        })
    }
}

impl Source for TrackSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
