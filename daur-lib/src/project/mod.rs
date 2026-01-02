//! Items pertaining to [`Project`].

pub mod track;

mod bar;
mod edit;
mod history;
mod manager;
mod renderer;
mod serial;
mod workspace;

pub use edit::Edit;
pub use manager::Manager;

#[doc(inline)]
pub use track::Track;

pub(crate) use bar::bar;
pub(crate) use history::HistoryEntry;
pub(crate) use renderer::Renderer;
pub(crate) use workspace::workspace;

use crate::Id;
use crate::NonZeroRatio;
use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::NonZeroDuration;
use crate::metre::TimeContext;
use crate::metre::TimeSignature;
use crate::note::Key;
use crate::project::track::Clip;
use crate::project::track::clip;
use crate::time::Tempo;
use arcstr::ArcStr;
use arcstr::literal;
use getset::CloneGetters;
use getset::Getters;
use indexmap::IndexMap;
use non_zero::non_zero;
use serde::Deserialize;
use serde::Serialize;
use serial::Serial;

const ADD_TRACK_LABEL: ArcStr = literal!("+");
const ADD_TRACK_DESCRIPTION: ArcStr = literal!("add track");
const DEFAULT_TRACK_TITLE: ArcStr = literal!("a track");

const DEFAULT_NOTES_DURATION: NonZeroDuration = NonZeroDuration {
    whole_notes: NonZeroRatio::integer(non_zero!(4)),
};

// TODO: Test that this isn't `Clone` (bc. id).
/// A musical piece consisting of multiple [tracks](Track).
#[derive(Debug, Default, Getters, CloneGetters, Deserialize)]
#[serde(from = "Serial")]
pub struct Project {
    /// The name of the project.
    #[get_clone = "pub"]
    name: ArcStr,

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

    /// Returns a mutable reference to a track.
    #[must_use]
    fn track_mut(&mut self, id: Id<Track>) -> Option<&mut Track> {
        self.tracks.get_mut(&id)
    }

    /// Returns a reference to a clip.
    #[must_use]
    pub(super) fn clip(&self, path: clip::Path) -> Option<(Instant, &Clip)> {
        self.track(path.track)?.clip(path.clip)
    }

    /// Returns a mutable reference to a clip.
    #[must_use]
    fn clip_mut(&mut self, path: clip::Path) -> Option<(Instant, &mut Clip)> {
        self.track_mut(path.track)?.clip_mut(path.clip)
    }

    /// Removes a clip from a track.
    #[must_use]
    fn remove_clip(&mut self, path: clip::Path) -> Option<(Instant, Clip)> {
        self.track_mut(path.track)?.remove_clip(path.clip)
    }

    pub(crate) fn time_context(&self) -> Changing<TimeContext> {
        &self.time_signature / &self.tempo
    }
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Serial::from(self).serialize(serializer)
    }
}
