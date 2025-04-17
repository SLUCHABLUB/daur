use crate::clip;
use std::collections::VecDeque;
use std::num::NonZeroU32;
use std::time::Duration;

/// An [audio source](rodio::Source) of a [track](crate::Track).
#[derive(Debug)]
pub struct Source {
    sample_rate: NonZeroU32,
    sample: usize,
    clips: VecDeque<(usize, clip::Source)>,
}

impl Source {
    /// Constructs a new source.
    #[must_use]
    pub fn new(
        sample_rate: NonZeroU32,
        clips: VecDeque<(usize, clip::Source)>,
        sample: usize,
    ) -> Source {
        Source {
            sample_rate,
            sample,
            clips,
        }
    }
}

impl Iterator for Source {
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

        self.sample = self.sample.wrapping_add(1);

        Some(if started {
            clip.next().unwrap_or(0.0)
        } else {
            0.0
        })
    }
}

impl rodio::Source for Source {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate.get()
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
