use crate::audio::sample;
use crate::metre::{Instant, NonZeroDuration, relative};
use crate::note::{Event, Note, Pitch};
use crate::project;
use crate::view::Context;
use clack_host::events::event_types::{NoteOffEvent, NoteOnEvent};
use clack_host::events::{Match, Pckn};
use saturating_cast::SaturatingCast as _;
use sorted_vec::SortedVec;
use std::cmp::min;
use std::collections::HashMap;

/// A sequence of musical notes.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Group {
    // INVARIANT: all notes are within `full_duration`
    // INVARIANT: notes are non-overlapping
    /// The notes in the group.
    // TODO: make this a `Dimap` when ids get added to `Note`
    notes: HashMap<(relative::Instant, Pitch), Note>,
    /// The duration of the whole note group.
    duration: NonZeroDuration,
}

impl Group {
    /// Constructs an empty note group.
    #[must_use]
    pub fn empty(duration: NonZeroDuration) -> Group {
        Group {
            notes: HashMap::new(),
            duration,
        }
    }

    /// Returns the duration of the note group.
    #[must_use]
    pub fn duration(&self) -> NonZeroDuration {
        self.duration
    }

    /// Tries inserting a note into the group.
    /// Does nothing if there is already a note at that position.
    /// Truncates the note if it goes outside the group or intersects another note.
    pub(crate) fn try_insert(&mut self, position: relative::Instant, pitch: Pitch, mut note: Note) {
        let end_of_group = relative::Instant {
            since_start: self.duration.get(),
        };

        // The start of the next note, or the end of the group.
        let next_position = self
            .with_pitch(pitch)
            .map(|(note_position, _)| note_position)
            .filter(|note_position| position < *note_position)
            .min()
            .unwrap_or(end_of_group);

        let max_duration = next_position - position;

        let Some(max_duration) = NonZeroDuration::from_duration(max_duration) else {
            // The note was outside the group or intersected another note.
            return;
        };

        note.duration = min(note.duration, max_duration);

        if let Some(last_note_end) = self
            .with_pitch(pitch)
            .map(|(note_position, _)| note_position)
            .filter(|note_position| *note_position < position)
            .max()
        {
            if position < last_note_end {
                return;
            }
        }

        self.notes.entry((position, pitch)).or_insert(note);
    }

    pub(crate) fn with_pitch(
        &self,
        pitch: Pitch,
    ) -> impl Iterator<Item = (relative::Instant, Note)> {
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
        start: Instant,
        project_settings: &project::Settings,
        sample_rate: sample::Rate,
    ) -> SortedVec<Event> {
        let mut events = Vec::new();

        #[expect(clippy::iter_over_hash_type, reason = "we sort the events")]
        for ((note_start, pitch), note) in &self.notes {
            let note_start = start + *note_start;

            let start = note_start.to_real_time(project_settings) * sample_rate;
            let end =
                (note_start + note.duration.get()).to_real_time(project_settings) * sample_rate;

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
