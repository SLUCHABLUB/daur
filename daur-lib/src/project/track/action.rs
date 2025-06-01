use crate::project::track::clip;
use std::collections::HashSet;
use std::path::PathBuf;

/// An action to take on a [track](super::Track).
#[derive(Clone, Debug)]
#[remain::sorted]
pub enum Action {
    /// Inserts an empty note clip into the selected track at the cursor.
    AddNotes,
    /// A clip action.
    Clip(clip::Action),
    /// Deletes some clips.
    DeleteClips(HashSet<clip::Id>),
    /// Imports an audio file into the selected track at the cursor.
    ImportAudio {
        /// The path to the file.
        file: PathBuf,
    },
}
