//! Items pertaining to [`Track`].

mod overview;
mod render_stream;
mod settings;

pub(crate) use overview::overview;
pub(crate) use render_stream::RenderStream;
pub(crate) use settings::settings;

use crate::audio::{Pair, SampleRate};
use crate::clip::Content;
use crate::metre::Instant;
use crate::project::Settings;
use crate::{Audio, Clip, Id};
use alloc::collections::BTreeMap;
use arcstr::{ArcStr, literal};
use getset::{CopyGetters, Getters, MutGetters};
use indexmap::IndexMap;
use std::collections::HashMap;

const DEFAULT_TITLE: ArcStr = literal!("a track");

/// A musical track.
// TODO: Test that this isn't `Clone` (bc. id).
#[doc(hidden)]
#[derive(Debug, Getters, MutGetters, CopyGetters)]
pub struct Track {
    // TODO: pub(super)
    #[get_copy = "pub(crate)"]
    id: Id<Track>,
    /// The name of the track.
    name: ArcStr,
    // TODO: use `Dimap<Instant, Id<Clip>, Clip, Bi<Btree, StdHash>, Index>`
    /// The clips in the track.
    clip_ids: BTreeMap<Instant, Id<Clip>>,
    clip_starts: HashMap<Id<Clip>, Instant>,
    clips: IndexMap<Id<Clip>, Clip>,
}

impl Track {
    /// Constructs a new, empty, track.
    #[must_use]
    pub(crate) fn new() -> Track {
        Track {
            id: Id::generate(),
            name: DEFAULT_TITLE,
            clip_ids: BTreeMap::new(),
            clip_starts: HashMap::new(),
            clips: IndexMap::new(),
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
            .clip_ids
            .last_key_value()
            .and_then(|(start, clip_id)| {
                let clip = &self.clips.get(clip_id)?;
                Some(clip.period(*start, settings).get().end())
            })
            .unwrap_or(Instant::START);
        let min_duration = min_end.to_real_time(settings).since_start;
        let min_len = (min_duration / sample_rate.sample_duration()).to_usize();

        let mut audio = Audio::empty(sample_rate);

        for (start, clip_id) in &self.clip_ids {
            let Some(clip) = self.clips.get(clip_id) else {
                continue;
            };

            let start = start.to_real_time(settings);
            // TODO: multiply by sample rate instead of dividing by sample duration
            let sample_offset = start.since_start / sample_rate.sample_duration();
            let sample_offset = sample_offset.to_usize();

            match clip.content() {
                Content::Audio(clip) => {
                    audio += &clip.as_audio().resample(sample_rate).offset(sample_offset);
                }
                Content::Notes(_) => {
                    // TODO: render notes
                }
            }
        }

        if audio.samples.len() < min_len {
            audio.samples.resize(min_len, Pair::ZERO);
        }

        RenderStream::new(audio)
    }

    // TODO: pub(super)
    /// Returns a reference to a clip.
    #[must_use]
    pub(crate) fn clip(&self, id: Id<Clip>) -> Option<(Instant, &Clip)> {
        let clip = self.clips.get(&id)?;
        let start = self.clip_starts.get(&id)?;
        Some((*start, clip))
    }

    // TODO: remove
    /// Returns a mutable reference to a clip.
    #[must_use]
    pub(crate) fn clip_mut(&mut self, id: Id<Clip>) -> Option<(Instant, &mut Clip)> {
        let clip = self.clips.get_mut(&id)?;
        let start = self.clip_starts.get(&id)?;
        Some((*start, clip))
    }

    // TODO: pub(super)
    pub(crate) fn try_insert_clip(&mut self, position: Instant, clip: Clip) -> Result<(), Clip> {
        if self.clips.contains_key(&clip.id()) {
            return Err(clip);
        }

        self.clip_ids.insert(position, clip.id());
        self.clip_starts.insert(clip.id(), position);
        self.clips.insert(clip.id(), clip);

        Ok(())
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}
