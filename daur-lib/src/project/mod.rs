//! Items pertaining to [`Project`].

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

pub(crate) use bar::bar;
pub(crate) use renderer::Renderer;
pub(crate) use workspace::workspace;

use crate::app::Selection;
use crate::audio::Player;
use crate::metre::Instant;
use crate::track::Track;
use crate::ui::{Grid, Length, NonZeroLength};
use crate::{Id, UserInterface, View};
use arcstr::{ArcStr, literal};
use getset::{CloneGetters, Getters};
use indexmap::IndexMap;

const ADD_TRACK_LABEL: ArcStr = literal!("+");
const ADD_TRACK_DESCRIPTION: ArcStr = literal!("add track");

// TODO: Test that this isn't `Clone` (bc. id).
/// A musical piece consisting of multiple [tracks](Track).
#[doc(hidden)]
#[derive(Debug, Default, Getters, CloneGetters)]
pub struct Project {
    /// The name of the project.
    #[get_clone = "pub"]
    title: ArcStr,

    /// The project settings.
    #[get = "pub(crate)"]
    settings: Settings,

    /// The tracks in the project.
    // TODO: remove getter / make pub(super)
    #[get = "pub(crate)"]
    tracks: IndexMap<Id<Track>, Track>,
}

impl Project {
    // TODO: pub(super)
    /// Returns a reference to a track.
    #[must_use]
    pub(crate) fn track(&self, id: Id<Track>) -> Option<&Track> {
        self.tracks.get(&id)
    }

    // TODO: remove (track edit)
    /// Returns a mutable reference to a track.
    #[must_use]
    pub(crate) fn track_mut(&mut self, id: Id<Track>) -> Option<&mut Track> {
        self.tracks.get_mut(&id)
    }

    pub(crate) fn bar<Ui: UserInterface>(
        &self,
        player: Option<Player>,
        edit_mode: bool,
        piano_roll_open: bool,
    ) -> View {
        bar::<Ui>(
            self.title(),
            &self.settings,
            player,
            edit_mode,
            piano_roll_open,
        )
    }

    pub(crate) fn workspace<Ui: UserInterface>(
        &self,
        track_settings_size: NonZeroLength,
        grid: Grid,
        overview_offset: Length,
        selection: &Selection,
        cursor: Instant,
        player: Option<&Player>,
    ) -> View {
        workspace::<Ui>(
            overview_offset,
            selection,
            track_settings_size,
            self.tracks.values(),
            self.settings.clone(),
            grid,
            cursor,
            player,
        )
    }
}
