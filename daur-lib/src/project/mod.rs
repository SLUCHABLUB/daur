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
use crate::{UserInterface, View};
use alloc::sync::{Arc, Weak};
use arcstr::{ArcStr, literal};
use getset::CloneGetters;

const ADD_TRACK_LABEL: ArcStr = literal!("+");
const ADD_TRACK_DESCRIPTION: ArcStr = literal!("add track");

/// A musical piece consisting of multiple [tracks](Track).
#[doc(hidden)]
#[derive(Clone, Debug, Default, CloneGetters)]
pub struct Project {
    /// The name of the project.
    #[get_clone = "pub"]
    pub title: ArcStr,

    /// The project settings.
    #[get_clone = "pub"]
    pub settings: Settings,

    /// The tracks in the project.
    pub tracks: Vec<Arc<Track>>,
}

impl Project {
    /// Returns a mutable reference to a track.
    #[must_use]
    pub fn track_mut(&mut self, weak: &Weak<Track>) -> Option<&mut Track> {
        self.tracks
            .iter_mut()
            .find(|arc| Arc::as_ptr(arc) == Weak::as_ptr(weak))
            .map(Arc::make_mut)
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
            self.tracks.clone(),
            self.settings.clone(),
            grid,
            cursor,
            player,
        )
    }
}
