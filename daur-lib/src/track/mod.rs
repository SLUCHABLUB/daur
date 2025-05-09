//! Items pertaining to [`Track`].

mod overview;
mod render_stream;
mod settings;

pub(crate) use overview::overview;
pub(crate) use render_stream::RenderStream;
pub(crate) use settings::settings;

use crate::audio::{Pair, SampleRate};
use crate::clip::Content;
use crate::musical_time::{Instant, Spaced};
use crate::project::Settings;
use crate::{Audio, Clip};
use arcstr::{ArcStr, literal};
use std::sync::{Arc, Weak};

const DEFAULT_TITLE: ArcStr = literal!("a track");

/// A musical track.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Track {
    /// The name of the track.
    pub name: ArcStr,
    /// The clips in the track.
    pub clips: Spaced<Arc<Clip>>,
}

impl Track {
    /// Constructs a new, empty, track.
    #[must_use]
    pub fn new() -> Track {
        Track {
            name: DEFAULT_TITLE,
            clips: Spaced::new(),
        }
    }

    /// Renders the track to audio.
    pub(crate) fn render_stream(
        &self,
        settings: &Settings,
        sample_rate: SampleRate,
    ) -> RenderStream {
        // TODO: remove when note processing has been added
        let min_end = self
            .clips
            .iter()
            .last()
            .map_or(Instant::START, |(start, clip)| {
                clip.content.period(start, settings).get().end()
            });
        let min_duration = min_end.to_real_time(settings).since_start;
        let min_len = (min_duration / sample_rate.sample_duration()).to_usize();

        let mut audio = Audio::empty(sample_rate);

        for (start, clip) in self.clips.iter() {
            let start = start.to_real_time(settings);
            // TODO: multiply by sample rate instead of dividing by sample duration
            let sample_offset = start.since_start / sample_rate.sample_duration();
            let sample_offset = sample_offset.to_usize();

            match &clip.content {
                Content::Audio(clip) => {
                    audio += &clip.as_audio().resample(sample_rate).offset(sample_offset);
                }
                Content::Notes(_) => {}
            }
        }

        if audio.samples.len() < min_len {
            audio.samples.resize(min_len, Pair::ZERO);
        }

        RenderStream::new(audio)
    }

    /// Returns a mutable reference to a clip.
    #[must_use]
    pub fn clip_mut(&mut self, weak: &Weak<Clip>) -> Option<&mut Clip> {
        self.clips
            .iter_mut()
            .map(|(_, clip)| clip)
            .find(|arc| Arc::as_ptr(arc) == Weak::as_ptr(weak))
            .map(Arc::make_mut)
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}
