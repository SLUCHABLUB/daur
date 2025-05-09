//! Types relating to [`Notes`].

mod chroma;
mod interval;
mod key;
mod key_interval;
mod note;
mod pitch;
mod sign;

pub use chroma::Chroma;
pub use interval::Interval;
pub use key::Key;
pub use key_interval::KeyInterval;
pub use note::Note;
pub use pitch::Pitch;
pub use sign::Sign;

use crate::metre::{NonZeroDuration, PitchSpaced};
use crate::view::Context;
use sign::{FLAT, SHARP};

/// A sequence of musical events.
/// Basically Midi.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Notes {
    // INVARIANT: all notes are within `full_duration`
    /// The notes in this clip, the instants are relative to the clip
    notes: PitchSpaced<Note>,
    full_duration: NonZeroDuration,
}

impl Notes {
    /// Constructs an empty clip.
    #[must_use]
    pub const fn empty(duration: NonZeroDuration) -> Notes {
        Notes {
            notes: PitchSpaced::new(),
            full_duration: duration,
        }
    }

    /// Returns the duration of the clip.
    #[must_use]
    pub fn duration(&self) -> NonZeroDuration {
        self.full_duration
    }

    pub(crate) fn draw_overview(&self, _context: &mut dyn Context) {
        // TODO: draw the notes
        let _: &Self = self;
    }
}
