//! Items pertaining to [`Project`].

mod action;
mod bar;
mod edit;
mod manager;
mod renderer;
mod workspace;

pub use action::Action;
pub use bar::bar;
pub use manager::Manager;
pub(crate) use renderer::Renderer;
pub(crate) use workspace::workspace;

use crate::audio::Player;
use crate::key::Key;
use crate::time::{Instant, Signature, Tempo};
use crate::track::Track;
use crate::ui::{Grid, Length, NonZeroLength};
use crate::{Changing, Clip, UserInterface, View, time, ui};
use arcstr::{ArcStr, literal};
use getset::CloneGetters;
use std::sync::{Arc, Weak};

const ADD_TRACK_LABEL: ArcStr = literal!("+");
const ADD_TRACK_DESCRIPTION: ArcStr = literal!("add track");

/// A musical piece consisting of multiple [tracks](Track).
#[doc(hidden)]
#[derive(Clone, Debug, Default, CloneGetters)]
pub struct Project {
    /// The name of the project
    #[get_clone = "pub"]
    pub title: ArcStr,

    /// The key of the project
    #[get_clone = "pub"]
    pub key: Arc<Changing<Key>>,
    /// The time signature of the project
    #[get_clone = "pub"]
    pub time_signature: Arc<Changing<Signature>>,
    // TODO: continuous change
    /// The tempo of the project
    #[get_clone = "pub"]
    pub tempo: Arc<Changing<Tempo>>,

    /// The tracks in the project
    pub tracks: Vec<Arc<Track>>,
}

impl Project {
    /// Returns the time mapping for the project.
    #[must_use]
    pub fn time_mapping(&self) -> time::Mapping {
        time::Mapping {
            tempo: self.tempo(),
            time_signature: self.time_signature(),
        }
    }

    /// Returns the ui mapping for the project.
    #[must_use]
    pub fn ui_mapping(&self, grid: Grid) -> ui::Mapping {
        ui::Mapping {
            time_signature: self.time_signature(),
            grid,
        }
    }

    /// Returns a mutable reference to a track.
    #[must_use]
    pub fn track_mut(&mut self, weak: &Weak<Track>) -> Option<&mut Track> {
        self.tracks
            .iter_mut()
            .find(|arc| Arc::as_ptr(arc) == Weak::as_ptr(weak))
            .map(Arc::make_mut)
    }

    pub(crate) fn bar<Ui: UserInterface>(&self, playing: bool) -> View {
        bar::<Ui>(
            self.title(),
            self.tempo.start,
            self.time_signature.start,
            self.key.start,
            playing,
        )
    }

    // TODO: merge `overview_offset` and `track_settings_width` into temporary settings and remove expect
    #[expect(clippy::too_many_arguments, reason = "todo")]
    pub(crate) fn workspace<Ui: UserInterface>(
        &self,
        track_settings_size: NonZeroLength,
        grid: Grid,
        overview_offset: Length,
        selected_track: &Weak<Track>,
        selected_clip: &Weak<Clip>,
        cursor: Instant,
        player: Option<&Player>,
    ) -> View {
        workspace::<Ui>(
            overview_offset,
            selected_track,
            selected_clip,
            track_settings_size,
            self.tracks.clone(),
            &self.time_mapping(),
            self.ui_mapping(grid),
            cursor,
            player,
        )
    }
}
