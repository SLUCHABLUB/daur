mod serial;

pub(crate) use serial::Serial;

use crate::Id;
use crate::Note;
use crate::audio::sample;
use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::NonZeroDuration;
use crate::metre::TimeContext;
use crate::metre::relative;
use crate::note::Event;
use crate::note::Pitch;
use crate::note::event::Sequence;
use crate::view::Painter;
use serde::Deserialize;
use std::cmp::min;
use std::collections::HashMap;
use thiserror::Error;

/// A sequence of musical notes.
#[derive(Eq, PartialEq, Debug, Deserialize)]
#[serde(try_from = "Serial")]
pub struct Group {
    // TODO: use a bimap
    // INVARIANT: all notes are within `full_duration`
    // INVARIANT: notes are non-overlapping
    /// The notes in the group.
    notes: HashMap<(relative::Instant, Pitch), Note>,
    note_positions: HashMap<Id<Note>, (relative::Instant, Pitch)>,
    /// The duration of the whole note group.
    duration: NonZeroDuration,
}

/// A note was not inserted.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
#[error("failed to insert a note into the note group")]
pub enum InsertionError {
    /// The note was inside another note.
    #[error("cannot insert a note inside another one")]
    InsideOther,
    /// The note was outside the clip.
    #[error("cannot insert a note outside the selected clip")]
    OutsideClip,
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

        if end_of_group <= position {
            return Err(InsertionError::OutsideClip);
        }

        // The start of the next note, or the end of the group.
        let next_position = self
            .with_pitch(pitch)
            .map(|(note_position, _)| note_position)
            .filter(|note_position| position < *note_position)
            .min()
            .unwrap_or(end_of_group);

        let max_duration = next_position - position;

        let Some(max_duration) = NonZeroDuration::from_duration(max_duration) else {
            return Err(InsertionError::InsideOther);
        };

        note.duration = min(note.duration, max_duration);

        if let Some(last_note_end) = self
            .with_pitch(pitch)
            .map(|(note_position, _)| note_position)
            .filter(|note_position| *note_position < position)
            .max()
            && position < last_note_end
        {
            return Err(InsertionError::InsideOther);
        }

        if self.notes.contains_key(&(position, pitch)) {
            return Err(InsertionError::InsideOther);
        }

        self.note_positions.insert(note.id, (position, pitch));
        self.notes.insert((position, pitch), note);

        Ok(())
    }

    pub(crate) fn remove(&mut self, note: Id<Note>) -> Option<(relative::Instant, Pitch, Note)> {
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
        let _: &Group = self;
        Box::new(|_| ())
    }

    pub(crate) fn to_events(
        &self,
        start: Instant,
        time_context: &Changing<TimeContext>,
        sample_rate: sample::Rate,
    ) -> Sequence {
        self.notes
            .iter()
            .flat_map(|((note_start, pitch), note)| {
                let id = note.id;
                let pitch = *pitch;

                let note_start = start + *note_start;

                let start = note_start * time_context * sample_rate;
                let end = (note_start + note.duration.get()) * time_context * sample_rate;

                [
                    (start, Event::NoteOn { id, pitch }),
                    (end, Event::NoteOff(id)),
                ]
            })
            .collect()
    }
}
