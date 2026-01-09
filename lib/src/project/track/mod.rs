//! Items pertaining to [`Track`].

pub mod clip;
mod overview;
mod serial;
mod settings;

#[doc(inline)]
pub use clip::Clip;

pub(super) use overview::Overview;
pub(super) use serial::Serial;
pub(crate) use settings::settings;

use crate::Audio;
use crate::Id;
use crate::audio::sample;
use crate::metre::Changing;
use crate::metre::Duration;
use crate::metre::Instant;
use crate::metre::TimeContext;
use crate::note::event::Sequence;
use crate::project::DEFAULT_TRACK_TITLE;
use arcstr::ArcStr;
use getset::CloneGetters;
use getset::CopyGetters;
use getset::Getters;
use getset::MutGetters;
use std::collections::BTreeMap;
use std::collections::HashMap;
use thiserror::Error;

/// An error occurred when trying to insert a clip.
#[derive(Debug)]
pub struct ClipInsertionError {
    // Boxed due to `clippy::result_large_err`.
    /// The clip that was attempted to be inserted.
    pub clip: Box<Clip>,
    /// The kind of error that occurred.
    pub kind: ClipInsertionErrorKind,
}

/// An error occurred when trying to insert a clip.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
pub enum ClipInsertionErrorKind {
    /// Tried inserting a clip at a position where there was already a clip.
    #[error("there is already a clip at that position")]
    PositionOccupied,
}

/// A musical track.
// TODO: Test that this isn't `Clone` (bc. id).
#[derive(Debug, Getters, MutGetters, CopyGetters, CloneGetters)]
pub struct Track {
    #[get_copy = "pub(super)"]
    id: Id<Track>,
    /// The name of the track.
    #[get_clone = "pub(super)"]
    name: ArcStr,
    // TODO: use a double-key map
    /// The clips in the track.
    clip_ids: BTreeMap<Instant, Id<Clip>>,
    clip_starts: HashMap<Id<Clip>, Instant>,
    clips: HashMap<Id<Clip>, Clip>,
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
    pub(super) fn clip(&self, id: Id<Clip>) -> Option<(Instant, &Clip)> {
        let clip = self.clips.get(&id)?;
        let start = self.clip_starts.get(&id)?;
        Some((*start, clip))
    }

    /// Returns a mutable reference to a clip.
    #[must_use]
    pub(super) fn clip_mut(&mut self, id: Id<Clip>) -> Option<(Instant, &mut Clip)> {
        let clip = self.clips.get_mut(&id)?;
        let start = self.clip_starts.get(&id)?;
        Some((*start, clip))
    }

    pub(super) fn remove_clip(&mut self, id: Id<Clip>) -> Option<(Instant, Clip)> {
        let start = self.clip_starts.remove(&id)?;
        self.clip_ids.remove(&start);
        let clip = self.clips.remove(&id)?;

        Some((start, clip))
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
        let minimum_duration = minimum_end.since_start * sample_rate;

        let mut audio = Audio::with_capacity(sample_rate, minimum_duration);

        for (start, clip_id) in &self.clip_ids {
            let Some(clip) = self.clips.get(clip_id) else {
                continue;
            };

            if let Some(clip) = clip.content().as_audio() {
                let clip_start = *start * time_context * sample_rate;

                audio.superpose_with_offset(&clip.audio, clip_start.since_start);
            }
        }

        audio.extend_to(minimum_duration);

        audio
    }

    pub(crate) fn events(
        &self,
        time_context: &Changing<TimeContext>,
        sample_rate: sample::Rate,
    ) -> Sequence {
        self.clip_ids
            .iter()
            .filter_map(|(start, clip_id)| Some((start, self.clips.get(clip_id)?)))
            .flat_map(|(start, clip)| {
                clip.events(*start, time_context, sample_rate)
                    .into_iterator()
            })
            .collect()
    }

    pub(super) fn try_insert_clip(
        &mut self,
        position: Instant,
        clip: Clip,
    ) -> Result<clip::Path, ClipInsertionError> {
        if self.clip_ids.contains_key(&position) {
            return Err(ClipInsertionError {
                clip: Box::new(clip),
                kind: ClipInsertionErrorKind::PositionOccupied,
            });
        }

        // TODO: check for overlap

        let clip_id = clip.id();

        self.clip_ids.insert(position, clip_id);
        self.clip_starts.insert(clip_id, position);
        self.clips.insert(clip_id, clip);

        Ok(clip::Path::new(self.id, clip_id))
    }
}

impl Default for Track {
    fn default() -> Track {
        Track::new()
    }
}
