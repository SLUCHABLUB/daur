//! Items pertaining to [`Project`].

mod action;
mod bar;
mod edit;
mod manager;
mod source;
mod workspace;

pub use action::Action;
pub use bar::bar;
pub use manager::Manager;
pub(crate) use workspace::workspace;

use crate::key::Key;
use crate::time::{Signature, Tempo};
use crate::track::Track;
use crate::ui::Grid;
use crate::{Changing, time, ui};
use arcstr::{ArcStr, literal};
use std::sync::Arc;

const ADD_TRACK_LABEL: ArcStr = literal!("+");
const ADD_TRACK_DESCRIPTION: ArcStr = literal!("add track");

/// A musical piece consisting of multiple [tracks](Track).
#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct Project {
    /// The name of the project
    pub title: ArcStr,

    /// The key of the project
    pub key: Arc<Changing<Key>>,
    /// The time signature of the project
    pub time_signature: Arc<Changing<Signature>>,
    // TODO: continuous change
    /// The tempo of the project
    pub tempo: Arc<Changing<Tempo>>,

    /// The tracks in the project
    pub tracks: Vec<Arc<Track>>,
}

impl Project {
    #[must_use]
    pub fn title(&self) -> ArcStr {
        ArcStr::clone(&self.title)
    }

    #[must_use]
    pub fn time_signature(&self) -> Arc<Changing<Signature>> {
        Arc::clone(&self.time_signature)
    }

    #[must_use]
    pub fn tempo(&self) -> Arc<Changing<Tempo>> {
        Arc::clone(&self.tempo)
    }

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
}
