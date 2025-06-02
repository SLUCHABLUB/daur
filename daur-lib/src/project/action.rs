use crate::metre::{Instant, NonZeroDuration};
use crate::note::{Key, Pitch};
use crate::project::track;
use crate::project::track::clip;
use std::collections::HashSet;
use std::path::PathBuf;

/// An action to take on a [project](super::Project).
#[derive(Clone, Debug)]
#[remain::sorted]
pub enum Edit {
    /// Adds a note to the selected clip.
    AddNote {
        /// The position of the note.
        position: Instant,
        /// The pitch of the note.
        pitch: Pitch,
        /// The note.
        duration: NonZeroDuration,
    },
    /// Inserts an empty note clip into the selected track at the cursor.
    AddNoteGroup,
    /// Adds an empty track.
    AddTrack,
    /// Deletes the selected item(s).
    Delete,
    /// Deletes some clips.
    DeleteClips(HashSet<clip::Id>),
    /// Deletes a track.
    DeleteTracks(HashSet<track::Id>),
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
