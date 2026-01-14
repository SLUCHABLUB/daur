//! Items pertaining to [`Event`].

mod sequence;
mod subsequence;

pub(crate) use sequence::Sequence;
pub(crate) use subsequence::Subsequence;

use crate::Id;
use crate::Note;
use crate::note::Pitch;

/// A note event (similar to MIDI).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) enum Event {
    /// Turns a note on.
    NoteOn {
        /// The id of the note to turn on.
        id: Id<Note>,
        /// The pitch of the note.
        pitch: Pitch,
    },
    /// Turns a note off.
    NoteOff(Id<Note>),
}
