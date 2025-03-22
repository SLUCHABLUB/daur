use crate::key::Key;
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
    /// Sets the key at the start of the project.
    SetDefaultKey(Key),
}
