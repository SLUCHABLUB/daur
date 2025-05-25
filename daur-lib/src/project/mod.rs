//! Items pertaining to [`Project`].

pub mod track;

mod action;
mod bar;
mod manager;
mod renderer;
mod settings;
mod workspace;

pub use action::Action;
pub use manager::Manager;
pub use settings::Settings;
use std::sync::Arc;
#[doc(inline)]
pub use track::Track;

pub(crate) use bar::bar;
pub(crate) use renderer::Renderer;
pub(crate) use workspace::workspace;

use crate::metre::{Instant, NonZeroInstant};
use crate::{Id, Selection};
use anyhow::Result;
use arcstr::{ArcStr, literal};
use getset::{CloneGetters, Getters};
use indexmap::IndexMap;
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

    /// The project settings.
    #[get = "pub(crate)"]
    settings: Settings,

    /// The tracks in the project.
    tracks: IndexMap<Id<Track>, Track>,
}

impl Project {
    /// Returns a reference to a track.
    #[must_use]
    pub(super) fn track(&self, id: Id<Track>) -> Option<&Track> {
        self.tracks.get(&id)
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
            Action::SetKey { instant, key } => {
                if let Some(position) = NonZeroInstant::from_instant(instant) {
                    Arc::make_mut(&mut self.settings.key)
                        .changes
                        .insert(position, key);
                } else {
                    Arc::make_mut(&mut self.settings.key).start = key;
                }
                Ok(())
            }
            Action::Track(action) => self
                .tracks
                .get_mut(&selection.track())
                .ok_or(NoTrackSelected)?
                .take_action(action, cursor, selection, &self.settings),
        }
    }
}
