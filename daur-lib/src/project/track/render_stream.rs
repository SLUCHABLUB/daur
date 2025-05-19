use crate::Audio;
use crate::audio::{Pair, SampleRate};

/// A struct used for rendering an audio track.
pub(crate) struct RenderStream {
    sample_index: usize,
    // this has been correctly resamples
    audio_input: Audio,
    // TODO: notes
    // TODO: nodes
}

impl RenderStream {
    pub(crate) fn new(audio_input: Audio) -> Self {
        RenderStream {
            sample_index: 0,
            audio_input,
        }
    }

    pub(crate) fn sample_rate(&self) -> SampleRate {
        self.audio_input.sample_rate
    }
}

impl Iterator for RenderStream {
    type Item = Pair;

    fn next(&mut self) -> Option<Pair> {
        // TODO process notes
        // TODO: use nodes
        let sample = *self.audio_input.samples.get(self.sample_index)?;

        self.sample_index = self.sample_index.saturating_add(1);

        Some(sample)
    }
}
