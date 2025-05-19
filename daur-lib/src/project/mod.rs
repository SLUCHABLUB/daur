//! Items pertaining to [`Project`].

pub mod track;

mod action;
mod bar;
mod edit;
mod manager;
mod renderer;
mod settings;
mod workspace;

pub use action::Action;
pub use manager::Manager;
pub use settings::Settings;
#[doc(inline)]
pub use track::Track;

pub(crate) use bar::bar;
pub(crate) use renderer::Renderer;
pub(crate) use workspace::workspace;

use crate::Id;
use arcstr::{ArcStr, literal};
use getset::{CloneGetters, Getters};
use indexmap::IndexMap;

const ADD_TRACK_LABEL: ArcStr = literal!("+");
const ADD_TRACK_DESCRIPTION: ArcStr = literal!("add track");

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

    // TODO: remove (track edit)
    /// Returns a mutable reference to a track.
    #[must_use]
    pub(crate) fn track_mut(&mut self, id: Id<Track>) -> Option<&mut Track> {
        self.tracks.get_mut(&id)
    }
}
