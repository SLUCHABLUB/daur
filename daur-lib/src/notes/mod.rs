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
use std::cmp::min;

use crate::metre::{Instant, NonZeroDuration, PitchSpaced};
use crate::view::Context;
use sign::{FLAT, SHARP};

/// A sequence of musical events.
/// Basically Midi.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Notes {
    // INVARIANT: all notes are within `full_duration`
    // INVARIANT: notes are non-overlapping
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

    /// Tries inserting a note into the clip.
    /// Does nothing if there is already a note at that position.
    /// Truncates the note if it goes outside the clip or intersects another note.
    pub fn try_insert(&mut self, position: Instant, pitch: Pitch, mut note: Note) {
        let max_duration = self.full_duration.get() - position.since_start;
        let Some(max_duration) = NonZeroDuration::from_duration(max_duration) else {
            // The note was outside the clip.
            return;
        };
        note.duration = min(note.duration, max_duration);
        // TODO: truncate notes on intersection

        let _note = self.notes.try_insert(position, pitch, note);
    }

    pub(crate) fn with_pitch(&self, pitch: Pitch) -> impl Iterator<Item = (Instant, Note)> {
        self.notes
            .with_pitch(pitch)
            .map(|(instant, note)| (instant, *note))
    }

    pub(crate) fn draw_overview(&self, _context: &mut dyn Context) {
        // TODO: draw the notes
        let _: &Self = self;
    }
}
