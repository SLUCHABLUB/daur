use crate::note::Note;
use crate::pitch::Pitch;
use crate::time::{Duration, Instant};
use crate::view::Context;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

/// A sequence of musical events.
/// Basically Midi.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Notes {
    // INVARIANT: all notes are within `full_duration`
    /// The notes in this clip, the instants are relative to the clip
    notes: BTreeMap<Instant, Note>,
    full_duration: Duration,
}

impl Notes {
    pub fn empty(duration: Duration) -> Notes {
        Notes {
            notes: BTreeMap::new(),
            full_duration: duration,
        }
    }

    pub fn duration(&self) -> Duration {
        self.full_duration
    }

    pub fn pitch_range(&self) -> Option<RangeInclusive<Pitch>> {
        let mut lowest = None;
        let mut highest = None;

        for note in self.notes.values() {
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
