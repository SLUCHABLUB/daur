use crate::metre::{NonZeroDuration, PitchSpaced};
use crate::note::Note;
use crate::pitch::Pitch;
use crate::view::Context;
use std::ops::RangeInclusive;

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
    pub const fn empty(duration: NonZeroDuration) -> Notes {
        Notes {
            notes: PitchSpaced::new(),
            full_duration: duration,
        }
    }

    pub fn duration(&self) -> NonZeroDuration {
        self.full_duration
    }

    pub fn pitch_range(&self) -> Option<RangeInclusive<Pitch>> {
        let mut lowest = None;
        let mut highest = None;

        for (_, _, note) in self.notes.iter() {
            if lowest.is_none_or(|lowest| note.pitch < lowest) {
                lowest = Some(note.pitch);
            }
            if highest.is_none_or(|highest| highest < note.pitch) {
                highest = Some(note.pitch);
            }
        }

        let lowest = lowest?;
        let highest = highest?;

        Some(RangeInclusive::new(lowest, highest))
    }

    pub fn draw_overview(&self, context: &mut dyn Context) {
        // TODO: draw the notes

        let _ = (self, context);
    }
}
