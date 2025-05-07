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

use crate::key::Key;
use crate::time::{Signature, Tempo};
use crate::track::Track;
use crate::ui::Grid;
use crate::{Changing, time, ui};
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
    #[must_use]
    pub fn time_mapping(&self) -> time::Mapping {
        time::Mapping {
            tempo: self.tempo(),
            time_signature: self.time_signature(),
        }
    }

    #[must_use]
    pub fn ui_mapping(&self, grid: Grid) -> ui::Mapping {
        ui::Mapping {
            time_signature: self.time_signature(),
            grid,
        }
    }

    #[must_use]
    pub fn track_mut(&mut self, weak: &Weak<Track>) -> Option<&mut Track> {
        self.tracks
            .iter_mut()
            .find(|arc| Arc::as_ptr(arc) == Weak::as_ptr(weak))
            .map(Arc::make_mut)
    }
}
