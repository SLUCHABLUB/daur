//! Types relating to [`Group`].

mod chroma;
mod event;
mod group;
mod interval;
mod key;
mod key_interval;
mod pitch;
mod sign;

pub use chroma::Chroma;
use getset::CopyGetters;
pub use group::Group;
pub use interval::Interval;
pub use key::Key;
pub use key_interval::KeyInterval;
pub use pitch::Pitch;
pub use sign::Sign;

pub(crate) use event::Event;
pub(crate) use group::NoteInsertionError;

use crate::Id;
use crate::metre::NonZeroDuration;
use sign::{FLAT, SHARP};

// TODO: Test that this isn't `Clone` (bc. id).
// TODO: pitch-bends?
/// A [note](https://en.wikipedia.org/wiki/Musical_note).
#[cfg_attr(doc, doc(hidden))]
#[derive(Eq, PartialEq, Debug, CopyGetters)]
#[expect(missing_copy_implementations, reason = "`Id`s should be unique")]
pub struct Note {
    #[get_copy = "pub(crate)"]
    id: Id<Note>,
    /// The duration of the note.
    #[get_copy = "pub(crate)"]
    duration: NonZeroDuration,
    // TODO: articulation
}

impl Note {
    pub(crate) fn new(duration: NonZeroDuration) -> Note {
        Note {
            id: Id::generate(),
            duration,
        }
    }
}
