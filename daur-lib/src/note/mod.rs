//! Types relating to [`Group`].

pub(crate) mod event;

mod chroma;
mod group;
mod id;
mod interval;
mod key;
mod key_interval;
mod pitch;
mod sign;

pub use chroma::Chroma;
use getset::CopyGetters;
pub use group::{Group, InsertionError};
pub use id::Id;
pub use interval::Interval;
pub use key::Key;
pub use key_interval::KeyInterval;
pub use pitch::Pitch;
pub use sign::Sign;

#[doc(inline)]
pub(crate) use event::Event;

use crate::metre::NonZeroDuration;
use crate::project::track::clip;
use sign::{FLAT, SHARP};

// TODO: Test that this isn't `Clone` (bc. id).
// TODO: pitch-bends?
/// A [note](https://en.wikipedia.org/wiki/Musical_note).
#[cfg_attr(doc, doc(hidden))]
#[derive(Eq, PartialEq, Debug, CopyGetters)]
#[expect(missing_copy_implementations, reason = "`Id`s should be unique")]
pub struct Note {
    #[get_copy = "pub(crate)"]
    id: Id,
    /// The duration of the note.
    #[get_copy = "pub(crate)"]
    duration: NonZeroDuration,
    // TODO: articulation
}

impl Note {
    pub(crate) fn new(duration: NonZeroDuration, clip: clip::Id) -> Note {
        Note {
            id: Id::generate(clip),
            duration,
        }
    }
}
