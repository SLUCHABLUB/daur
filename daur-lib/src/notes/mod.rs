//! Types relating to [`Notes`].

mod chroma;
mod event;
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

pub(crate) use event::Event;

use crate::audio::sample;
use crate::metre::{Instant, NonZeroDuration};
use crate::notes::sign::{FLAT, SHARP};
use crate::project::Settings;
use crate::view::Context;
use clack_host::events::event_types::{NoteOffEvent, NoteOnEvent};
use clack_host::events::{Match, Pckn};
use saturating_cast::SaturatingCast as _;
use sorted_vec::SortedVec;
use std::cmp::min;
use std::collections::HashMap;

/// A sequence of musical events.
/// Basically Midi.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Notes {
    // INVARIANT: all notes are within `full_duration`
    // INVARIANT: notes are non-overlapping
    /// The notes in this clip, the instants are relative to the clip
    notes: HashMap<(Instant, Pitch), Note>,
    full_duration: NonZeroDuration,
}

impl Notes {
    /// Constructs an empty clip.
    #[must_use]
    pub fn empty(duration: NonZeroDuration) -> Notes {
        Notes {
            notes: HashMap::new(),
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
    pub(crate) fn try_insert(&mut self, position: Instant, pitch: Pitch, mut note: Note) {
        let max_duration = self.full_duration.get() - position.since_start;
        let Some(max_duration) = NonZeroDuration::from_duration(max_duration) else {
            // The note was outside the clip.
            return;
        };
        note.duration = min(note.duration, max_duration);
        // TODO: truncate notes on intersection

        self.notes.entry((position, pitch)).or_insert(note);
    }

    pub(crate) fn with_pitch(&self, pitch: Pitch) -> impl Iterator<Item = (Instant, Note)> {
        self.notes
            .iter()
            .filter_map(move |((instant, note_pitch), note)| {
                (pitch == *note_pitch).then_some((*instant, *note))
            })
    }

    pub(crate) fn draw_overview(&self, _context: &mut dyn Context) {
        // TODO: draw the notes
        let _: &Self = self;
    }

    pub(crate) fn to_events(
        &self,
        clip_start: Instant,
        settings: &Settings,
        sample_rate: sample::Rate,
    ) -> SortedVec<Event> {
        let mut events = Vec::new();

        #[expect(clippy::iter_over_hash_type, reason = "we sort the events")]
        for ((instant, pitch), note) in &self.notes {
            let instant = clip_start + instant.since_start;

            let start = instant.to_real_time(settings) * sample_rate;
            let end = (instant + note.duration.get()).to_real_time(settings) * sample_rate;

            let Some(key) = pitch.midi_number() else {
                continue;
            };

            // TODO: add an id to `Note`
            let tuple = Pckn {
                port_index: Match::Specific(0),
                channel: Match::All,
                key: Match::Specific(key.into()),
                note_id: Match::All,
            };

            // TODO: take the velocity from the note
            let on = NoteOnEvent::new(start.since_start.samples.saturating_cast(), tuple, 0.5);
            let off = NoteOffEvent::new(end.since_start.samples.saturating_cast(), tuple, 0.5);

            events.push(Event::NoteOn(on));
            events.push(Event::NoteOff(off));
        }

        SortedVec::from_unsorted(events)
    }
}
