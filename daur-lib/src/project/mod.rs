//! Items pertaining to [`Project`].

pub mod track;

mod action;
mod bar;
mod manager;
mod renderer;
mod workspace;

pub use action::Action;
pub use manager::Manager;
#[doc(inline)]
pub use track::Track;

pub(crate) use bar::bar;
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
use indexmap::map::Entry as IndexEntry;
use std::collections::hash_map::Entry as StdEntry;
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

    // TODO: return a history entry
    #[remain::check]
    pub(crate) fn take_action(
        &mut self,
        action: Action,
        cursor: Instant,
        selection: &mut Selection,
    ) -> Result<()> {
        #[sorted]
        match action {
            Action::AddTrack => {
                let track = Track::new();
                selection.set_track(track.id());
                self.tracks.insert(track.id(), track);
                Ok(())
            }
            Action::Delete => {
                let IndexEntry::Occupied(mut track) = self.tracks.entry(selection.track()) else {
                    return Ok(());
                };

                let StdEntry::Occupied(clip) = track.get_mut().clips_mut().entry(selection.clip())
                else {
                    track.shift_remove();
                    return Ok(());
                };

                // TODO: check if a note is selected
                clip.remove();

                Ok(())
            }
            Action::SetKey { instant, key } => {
                if let Some(position) = NonZeroInstant::from_instant(instant) {
                    self.key.changes.insert(position, key);
                } else {
                    self.key.start = key;
                }
                Ok(())
            }
            Action::Track(action) => {
                let time_context = self.time_context();

                self.tracks
                    .get_mut(&selection.track())
                    .ok_or(NoTrackSelected)?
                    .take_action(action, cursor, selection, &time_context)
            }
        }
    }
}
