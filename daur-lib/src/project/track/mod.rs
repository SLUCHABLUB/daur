//! Items pertaining to [`Track`].

pub mod clip;
mod id;
mod overview;
mod settings;

#[doc(inline)]
pub use clip::Clip;
pub use id::Id;

pub(crate) use overview::overview;
pub(crate) use settings::settings;

use crate::Audio;
use crate::audio::sample;
use crate::audio::sample::Pair;
use crate::metre::{Changing, Duration, Instant, TimeContext};
use crate::note::Event;
use crate::project::DEFAULT_TRACK_TITLE;
use arcstr::ArcStr;
use getset::{CopyGetters, Getters, MutGetters};
use sorted_vec::SortedVec;
use std::collections::{BTreeMap, HashMap};
use thiserror::Error;

/// An error occurred when trying to insert a clip.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
pub enum ClipInsertionError {
    /// Tried inserting a clip at a position where there was already a clip.
    #[error("there is already a clip at that position")]
    PositionOccupied,
}

/// A musical track.
// TODO: Test that this isn't `Clone` (bc. id).
#[cfg_attr(doc, doc(hidden))]
#[derive(Debug, Getters, MutGetters, CopyGetters)]
pub struct Track {
    #[get_copy = "pub(super)"]
    id: Id,
    /// The name of the track.
    name: ArcStr,
    // TODO: use `Dimap<Instant, Id<Clip>, Clip, Bi<Btree, StdHash>, StdHash>`
    /// The clips in the track.
    clip_ids: BTreeMap<Instant, clip::Id>,
    clip_starts: HashMap<clip::Id, Instant>,
    // TODO: remove getter
    #[get_mut = "pub(super)"]
    clips: HashMap<clip::Id, Clip>,
}

impl Track {
    /// Constructs a new, empty, track.
    #[must_use]
    pub(crate) fn new() -> Track {
        Track {
            id: Id::generate(),
            name: DEFAULT_TRACK_TITLE,
            clip_ids: BTreeMap::new(),
            clip_starts: HashMap::new(),
            clips: HashMap::new(),
        }
    }

    /// Returns a reference to a clip.
    #[must_use]
    pub(super) fn clip(&self, id: clip::Id) -> Option<(Instant, &Clip)> {
        let clip = self.clips.get(&id)?;
        let start = self.clip_starts.get(&id)?;
        Some((*start, clip))
    }

    /// Returns a reference to a clip.
    #[must_use]
    pub(super) fn clip_mut(&mut self, id: clip::Id) -> Option<(Instant, &mut Clip)> {
        let clip = self.clips.get_mut(&id)?;
        let start = self.clip_starts.get(&id)?;
        Some((*start, clip))
    }

    fn minimum_duration(&self) -> Duration {
        let Some((start, clip_id)) = self.clip_ids.last_key_value() else {
            return Duration::ZERO;
        };

        let Some(clip) = self.clips.get(clip_id) else {
            return Duration::ZERO;
        };

        (*start + clip.duration().get()).since_start
    }

    pub(crate) fn audio_sum(
        &self,
        time_context: &Changing<TimeContext>,
        sample_rate: sample::Rate,
    ) -> Audio {
        let minimum_end = Instant {
            since_start: self.minimum_duration(),
        };
        let minimum_end = minimum_end * time_context;
        let minimum_end = minimum_end.since_start * sample_rate;
        let minimum_sample_count = minimum_end.samples;

        let mut audio = Audio {
            sample_rate,
            samples: vec![Pair::ZERO; minimum_sample_count],
        };

        for (start, clip_id) in &self.clip_ids {
            let Some(clip) = self.clips.get(clip_id) else {
                continue;
            };

            if let Some(clip) = clip.content().as_audio() {
                let clip_start = *start * time_context * sample_rate;

                audio.add_assign_at(&clip.audio, clip_start.since_start);
            }
        }

        audio
    }

    pub(crate) fn events(
        &self,
        time_context: &Changing<TimeContext>,
        sample_rate: sample::Rate,
    ) -> SortedVec<Event> {
        let mut events = SortedVec::new();

        for (start, clip_id) in &self.clip_ids {
            let Some(clip) = self.clips.get(clip_id) else {
                continue;
            };

            events.extend(clip.events(*start, time_context, sample_rate));
        }

        events
    }

    pub(super) fn try_insert_clip(
        &mut self,
        position: Instant,
        clip: Clip,
    ) -> Result<(), ClipInsertionError> {
        if self.clip_ids.contains_key(&position) {
            return Err(ClipInsertionError::PositionOccupied);
        }

        // TODO: check for overlap

        self.clip_ids.insert(position, clip.id());
        self.clip_starts.insert(clip.id(), position);
        self.clips.insert(clip.id(), clip);

        Ok(())
    }

    // TODO: replace with a pub(super) mut-getter for the dimap
    pub(super) fn remove_clip(&mut self, id: clip::Id) -> Option<(Instant, Clip)> {
        let start = self.clip_starts.remove(&id)?;
        self.clip_ids.remove(&start);
        let clip = self.clips.remove(&id)?;

        Some((start, clip))
    }
}

impl Default for Track {
    fn default() -> Track {
        Track::new()
    }
}
