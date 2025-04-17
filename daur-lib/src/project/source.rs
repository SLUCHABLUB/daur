use crate::track::Source;
use std::num::NonZeroU32;
use std::time::Duration;

#[derive(Debug)]
pub struct ProjectSource {
    pub sample_rate: NonZeroU32,
    pub tracks: Vec<Source>,
}

impl Iterator for ProjectSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = 0.0;
        let mut none_count: usize = 0;

        for track in &mut self.tracks {
            if let Some(sample) = track.next() {
                sum += sample;
            } else {
                none_count = none_count.saturating_add(1);
            }
        }

        if none_count == self.tracks.len() {
            None
        } else {
            Some(sum.clamp(-1.0, 1.0))
        }
    }
}

impl rodio::Source for ProjectSource {
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
