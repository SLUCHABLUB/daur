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
pub use group::Group;
pub use interval::Interval;
pub use key::Key;
pub use key_interval::KeyInterval;
pub use pitch::Pitch;
pub use sign::Sign;

pub(crate) use event::Event;

use crate::metre::NonZeroDuration;
use sign::{FLAT, SHARP};

// TODO: pitch-bends?
/// A [note](https://en.wikipedia.org/wiki/Musical_note).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(doc, doc(hidden))]
pub struct Note {
    /// The duration of the note.
    pub duration: NonZeroDuration,
    // TODO: articulation
}
