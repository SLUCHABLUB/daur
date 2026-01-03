//! Types relating to [`Group`].

pub(crate) mod event;
pub(crate) mod group;

mod id;
mod interval;
mod key;
mod non_unison_simple_interval;
mod pitch;
mod pitch_class;
mod serial;
mod sign;

use getset::CopyGetters;
pub use group::Group;
pub use group::InsertionError;
pub use id::Path;
pub use interval::Interval;
pub use key::Key;
pub use non_unison_simple_interval::NonUnisonSimpleInterval;
pub use pitch::Pitch;
pub use pitch_class::PitchClass;
pub use sign::Sign;

#[doc(inline)]
pub(crate) use event::Event;
pub(crate) use serial::Serial;

use crate::Id;
use crate::metre::NonZeroDuration;
use sign::FLAT;
use sign::SHARP;

// TODO: Test that this isn't `Clone` (bc. id).
// TODO: pitch-bends?
/// A [note](https://en.wikipedia.org/wiki/Musical_note).
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
