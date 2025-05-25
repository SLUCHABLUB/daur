use crate::metre::Instant;
use crate::note::{Note, Pitch};

/// An action to take on a [clip](super::Clip).
#[derive(Copy, Clone, Debug)]
#[remain::sorted]
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
}
