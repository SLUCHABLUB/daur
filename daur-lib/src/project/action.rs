use crate::key::Key;
use crate::musical_time::Instant;
use std::path::PathBuf;

/// An action to take on an [app](crate::App).
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Action {
    /// Inserts an empty note clip into the selected track at the cursor.
    AddNotes,
    /// Adds an empty track.
    AddTrack,
    /// Imports an audio file into the selected track at the cursor.
    ImportAudio {
        /// The path to the file.
        file: PathBuf,
    },
    /// Sets the key at an instant in the project.
    SetKey {
        /// The instant of the key change.
        instant: Instant,
        /// The new key.
        key: Key,
    },
}
