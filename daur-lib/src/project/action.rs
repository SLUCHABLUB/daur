use crate::Id;
use crate::metre::Instant;
use crate::note::Key;
use crate::project::{Track, track};

/// An action to take on a [project](super::Project).
#[derive(Clone, Debug)]
#[remain::sorted]
pub enum Action {
    /// Adds an empty track.
    AddTrack,
    /// Deletes the selected item(s).
    Delete,
    /// Deletes a track.
    DeleteTrack(Id<Track>),
    /// Sets the key at an instant in the project.
    SetKey {
        /// The instant of the key change.
        instant: Instant,
        /// The new key.
        key: Key,
    },
    /// A track action.
    Track(track::Action),
}
