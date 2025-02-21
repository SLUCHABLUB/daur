use crate::key::Key;
use std::path::PathBuf;

#[derive(Clone, Eq, PartialEq)]
pub enum Action {
    /// Inserts an empty `Notes` clip into the selected track at the cursor
    AddNotes,
    /// Adds an empty track
    AddTrack,
    /// Imports an audio file into the selected track at the cursor
    ImportAudio {
        file: PathBuf,
    },
    SetDefaultKey(Key),
}
