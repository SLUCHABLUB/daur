//! Items pertaining to [`Project`].

pub mod track;

mod action;
mod bar;
mod history;
mod manager;
mod renderer;
mod workspace;

pub use action::Action;
pub use manager::Manager;
#[doc(inline)]
pub use track::Track;

pub(crate) use bar::bar;
pub(crate) use history::HistoryEntry;
pub(crate) use renderer::Renderer;
pub(crate) use workspace::workspace;

use crate::metre::{Changing, Instant, NonZeroInstant, TimeContext, TimeSignature};
use crate::note::Key;
use crate::time::Tempo;
use crate::{Id, Selection};
use anyhow::Result;
use arcstr::{ArcStr, literal};
use getset::{CloneGetters, Getters};
use indexmap::IndexMap;
use std::mem::{replace, take};
use thiserror::Error;

const ADD_TRACK_LABEL: ArcStr = literal!("+");
const ADD_TRACK_DESCRIPTION: ArcStr = literal!("add track");

#[derive(Debug, Error)]
#[error("no track is selected")]
struct NoTrackSelected;

// TODO: Test that this isn't `Clone` (bc. id).
/// A musical piece consisting of multiple [tracks](Track).
#[cfg_attr(doc, doc(hidden))]
#[derive(Debug, Default, Getters, CloneGetters)]
pub struct Project {
    /// The name of the project.
    #[get_clone = "pub"]
    title: ArcStr,

    // TODO: continuous change
    /// The tempo of the project
    tempo: Changing<Tempo>,
    /// The time signature of the project.
    #[get = "pub(crate)"]
    time_signature: Changing<TimeSignature>,
    /// The key of the project.
    #[get = "pub(crate)"]
    key: Changing<Key>,

    /// The tracks in the project.
    tracks: IndexMap<Id<Track>, Track>,
}

impl Project {
    /// Returns a reference to a track.
    #[must_use]
    pub(super) fn track(&self, id: Id<Track>) -> Option<&Track> {
        self.tracks.get(&id)
    }

    pub(crate) fn time_context(&self) -> Changing<TimeContext> {
        &self.time_signature / &self.tempo
    }

    #[remain::check]
    pub(crate) fn take_action(
        &mut self,
        action: Action,
        cursor: Instant,
        selection: &mut Selection,
    ) -> Result<Option<HistoryEntry>> {
        #[sorted]
        match action {
            Action::AddTrack => {
                let track = Track::new();
                let id = track.id();

                selection.track = id;
                self.tracks.insert(id, track);

                Ok(Some(HistoryEntry::AddTrack(id)))
            }
            Action::Delete => {
                let action = if selection.clips.is_empty() {
                    Action::DeleteTrack(selection.track)
                } else {
                    let action = if selection.notes.is_empty() {
                        track::Action::DeleteClips(take(&mut selection.clips))
                    } else {
                        // TODO: delete notes
                        return Ok(None);
                    };

                    Action::Track(action)
                };

                self.take_action(action, cursor, selection)
            }
            Action::DeleteTrack(track) => {
                let Some(index) = self.tracks.get_index_of(&track) else {
                    return Ok(None);
                };
                let Some(track) = self.tracks.shift_remove(&track) else {
                    return Ok(None);
                };

                Ok(Some(HistoryEntry::DeleteTrack { index, track }))
            }
            Action::SetKey { instant, key } => {
                let old = if let Some(position) = NonZeroInstant::from_instant(instant) {
                    self.key.changes.insert(position, key)
                } else {
                    Some(replace(&mut self.key.start, key))
                };

                Ok(Some(HistoryEntry::SetKey {
                    at: instant,
                    to: key,
                    from: old,
                }))
            }
            Action::Track(action) => {
                let time_context = self.time_context();

                self.tracks
                    .get_mut(&selection.track)
                    .ok_or(NoTrackSelected)?
                    .take_action(action, cursor, selection, &time_context)
            }
        }
    }
}
