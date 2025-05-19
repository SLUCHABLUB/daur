use crate::metre::Instant;
use crate::notes::{Key, Note, Pitch};
use std::path::PathBuf;

/// An action to take on an [app](crate::App).
#[derive(Clone, Debug)]
pub enum Action {
    /// Adds a note to the selected.
    AddNote {
        /// The position of the note.
        position: Instant,
        /// The pitch of the note.
        pitch: Pitch,
        /// The note.
        note: Note,
    },
    /// Inserts an empty note clip into the selected track at the cursor.
    AddNotes,
    // TODO: select the newly added track
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
