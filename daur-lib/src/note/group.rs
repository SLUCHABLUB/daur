use crate::audio::sample;
use crate::metre::{Changing, Instant, NonZeroDuration, TimeContext, relative};
use crate::note;
use crate::note::{Event, Note, Pitch};
use crate::view::Painter;
use clack_host::events::event_types::{NoteOffEvent, NoteOnEvent};
use clack_host::events::{Match, Pckn};
use saturating_cast::SaturatingCast as _;
use sorted_vec::SortedVec;
use std::cmp::min;
use std::collections::HashMap;
use thiserror::Error;

// TODO: make more informative
/// A note was not inserted.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
#[error("failed to insert a note into the note group")]
pub struct InsertionError;

/// A sequence of musical notes.
#[derive(Eq, PartialEq, Debug)]
pub struct Group {
    // TODO: use a dimap
    // INVARIANT: all notes are within `full_duration`
    // INVARIANT: notes are non-overlapping
    /// The notes in the group.
    notes: HashMap<(relative::Instant, Pitch), Note>,
    note_positions: HashMap<note::Id, (relative::Instant, Pitch)>,
    /// The duration of the whole note group.
    duration: NonZeroDuration,
}

impl Group {
    /// Constructs an empty note group.
    #[must_use]
    pub fn empty(duration: NonZeroDuration) -> Group {
        Group {
            notes: HashMap::new(),
            note_positions: HashMap::new(),
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
    pub(crate) fn try_insert(
        &mut self,
        position: relative::Instant,
        pitch: Pitch,
        mut note: Note,
    ) -> Result<(), InsertionError> {
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
            return Err(InsertionError);
        };

        note.duration = min(note.duration, max_duration);

        if let Some(last_note_end) = self
            .with_pitch(pitch)
            .map(|(note_position, _)| note_position)
            .filter(|note_position| *note_position < position)
            .max()
        {
            if position < last_note_end {
                return Err(InsertionError);
            }
        }

        if self.notes.contains_key(&(position, pitch)) {
            return Err(InsertionError);
        }

        self.note_positions.insert(note.id, (position, pitch));
        self.notes.insert((position, pitch), note);

        Ok(())
    }

    pub(crate) fn remove(&mut self, note: note::Id) -> Option<(relative::Instant, Pitch, Note)> {
        let position = self.note_positions.remove(&note)?;
        let note = self.notes.remove(&position)?;

        let (instant, pitch) = position;
        Some((instant, pitch, note))
    }

    pub(crate) fn with_pitch(
        &self,
        pitch: Pitch,
    ) -> impl Iterator<Item = (relative::Instant, &Note)> {
        self.notes
            .iter()
            .filter_map(move |((instant, note_pitch), note)| {
                (pitch == *note_pitch).then_some((*instant, note))
            })
    }

    pub(crate) fn overview_painter(&self) -> Box<Painter> {
        // TODO: draw the notes
        let _: &Self = self;
        Box::new(|_| ())
    }

    pub(crate) fn to_events(
        &self,
        start: Instant,
        time_context: &Changing<TimeContext>,
        sample_rate: sample::Rate,
    ) -> SortedVec<Event> {
        let mut events = Vec::new();

        #[expect(clippy::iter_over_hash_type, reason = "we sort the events")]
        for ((note_start, pitch), note) in &self.notes {
            let note_start = start + *note_start;

            let start = note_start * time_context * sample_rate;
            let end = (note_start + note.duration.get()) * time_context * sample_rate;

            let tuple = Pckn {
                port_index: Match::Specific(0),
                channel: Match::All,
                key: Match::Specific(u16::from(pitch.midi_number())),
                note_id: Match::Specific(note.id.to_u32()),
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
