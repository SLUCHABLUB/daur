use crate::project::track::clip;
use std::path::PathBuf;

/// An action to take on a [track](super::Track).
#[derive(Clone, Debug)]
pub enum Action {
    /// A clip action.
    Clip(clip::Action),
    /// Inserts an empty note clip into the selected track at the cursor.
    AddNotes,
    /// Imports an audio file into the selected track at the cursor.
    ImportAudio {
        /// The path to the file.
        file: PathBuf,
    },
}
