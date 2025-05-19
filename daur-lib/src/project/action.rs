use crate::metre::Instant;
use crate::notes::Key;
use crate::project::track;

/// An action to take on a [project](super::Project).
#[derive(Clone, Debug)]
pub enum Action {
    /// A track action.
    Track(track::Action),
    // TODO: select the newly added track
    /// Adds an empty track.
    AddTrack,
    /// Sets the key at an instant in the project.
    SetKey {
        /// The instant of the key change.
        instant: Instant,
        /// The new key.
        key: Key,
    },
}
