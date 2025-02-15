use crate::track::TrackSource;
use rodio::Source;
use std::time::Duration;

pub struct ProjectSource {
    pub sample_rate: u32,
    pub tracks: Vec<TrackSource>,
}

impl Iterator for ProjectSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = 0.0;
        let mut none_count = 0;

        for track in &mut self.tracks {
            if let Some(sample) = track.next() {
                sum += sample;
            } else {
                none_count += 1;
            }
        }

        if none_count == self.tracks.len() {
            None
        } else {
            Some(sum.clamp(-1.0, 1.0))
        }
    }
}

impl Source for ProjectSource {
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
