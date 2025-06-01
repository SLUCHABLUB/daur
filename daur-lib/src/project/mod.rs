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
use crate::project::track::{Clip, clip};
use crate::select::Selection;
use crate::time::Tempo;
use anyhow::Result;
use arcstr::{ArcStr, literal};
use getset::{CloneGetters, Getters};
use indexmap::IndexMap;
use mitsein::iter1::IteratorExt as _;
use std::mem::replace;
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
    tracks: IndexMap<track::Id, Track>,
}

impl Project {
    /// Returns a reference to a track.
    #[must_use]
    pub(super) fn track(&self, id: track::Id) -> Option<&Track> {
        self.tracks.get(&id)
    }

    /// Returns a reference to a clip.
    #[must_use]
    pub(super) fn clip(&self, id: clip::Id) -> Option<(Instant, &Clip)> {
        self.track(id.track())?.clip(id)
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

                selection.push_track(id);
                self.tracks.insert(id, track);

                Ok(Some(HistoryEntry::AddTrack(id)))
            }
            Action::Delete => {
                let action = if let Some(_notes) = selection.take_notes() {
                    // TODO: delete notes
                    return Ok(None);
                } else if let Some(clips) = selection.take_clips() {
                    Action::Track(track::Action::DeleteClips(clips))
                } else if let Some(tracks) = selection.take_tracks() {
                    Action::DeleteTracks(tracks)
                } else {
                    // Nothing is selected.
                    return Ok(None);
                };

                self.take_action(action, cursor, selection)
            }
            Action::DeleteTracks(tracks) => Ok(tracks
                .into_iter()
                .filter_map(|track| {
                    let index = self.tracks.get_index_of(&track)?;
                    let track = self.tracks.shift_remove(&track)?;

                    Some((index, track))
                })
                .try_collect1()
                .ok()
                .map(HistoryEntry::DeleteTracks)),
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

                let Some(track) = selection.top_track() else {
                    return Ok(None);
                };

                self.tracks
                    .get_mut(&track)
                    .ok_or(NoTrackSelected)?
                    .take_action(action, cursor, selection, &time_context)
            }
        }
    }
}
