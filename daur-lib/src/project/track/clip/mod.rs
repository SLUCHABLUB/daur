//! Items pertaining to [`Clip`].

mod action;
mod content;
mod id;
mod overview;

pub use action::Action;
pub use content::Content;
pub use id::Id;

pub(crate) use overview::overview;

use crate::audio::{FixedLength, sample};
use crate::metre::{Changing, Instant, NonZeroDuration, TimeContext};
use crate::note::{Event, NoteInsertionError};
use crate::project::{HistoryEntry, track};
use crate::ui::Colour;
use crate::{Note, note};
use anyhow::Result;
use arcstr::{ArcStr, literal};
use getset::{CloneGetters, CopyGetters, Getters, MutGetters};
use sorted_vec::SortedVec;
use thiserror::Error;

const DEFAULT_AUDIO_COLOUR: Colour = Colour {
    red: 0,
    green: 255,
    blue: 0,
};

const DEFAULT_NOTES_NAME: ArcStr = literal!("some notes");
const DEFAULT_NOTES_COLOUR: Colour = Colour {
    red: 255,
    green: 0,
    blue: 255,
};

#[derive(Debug, Error)]
#[error("the selected clip is not a notes-clip")]
struct NoNotesSelected;

/// A part of a [track](super::Track).
// TODO: Test that this isn't `Clone` (bc. id).
#[cfg_attr(doc, doc(hidden))]
#[derive(Debug, Getters, MutGetters, CopyGetters, CloneGetters)]
pub struct Clip {
    #[get_copy = "pub(super)"]
    id: Id,
    /// The name of the clip.
    #[get_clone = "pub"]
    name: ArcStr,
    /// The colour of the clip.
    #[get_copy = "pub"]
    colour: Colour,
    /// The content of the clip.
    #[get = "pub(crate)"]
    content: Content,
}

impl Clip {
    #[must_use]
    pub(crate) fn from_audio(name: ArcStr, audio: FixedLength, track: track::Id) -> Clip {
        Clip {
            id: Id::generate(track),
            name,
            colour: DEFAULT_AUDIO_COLOUR,
            content: Content::Audio(audio),
        }
    }

    #[must_use]
    pub(crate) fn empty_notes(duration: NonZeroDuration, track: track::Id) -> Clip {
        Clip {
            id: Id::generate(track),
            name: DEFAULT_NOTES_NAME,
            colour: DEFAULT_NOTES_COLOUR,
            content: Content::Notes(note::Group::empty(duration)),
        }
    }

    pub(crate) fn duration(&self) -> NonZeroDuration {
        self.content.duration()
    }

    pub(crate) fn events(
        &self,
        clip_start: Instant,
        time_context: &Changing<TimeContext>,
        sample_rate: sample::Rate,
    ) -> SortedVec<Event> {
        let Some(notes) = self.content.as_notes() else {
            return SortedVec::new();
        };

        notes.to_events(clip_start, time_context, sample_rate)
    }

    #[remain::check]
    pub(crate) fn take_action(
        &mut self,
        clip_position: Instant,
        action: Action,
    ) -> Result<Option<HistoryEntry>> {
        #[sorted]
        match action {
            Action::AddNote {
                position: note_position,
                pitch,
                mut duration,
            } => {
                if note_position < clip_position {
                    let difference = clip_position - note_position;
                    let Some(max_duration) =
                        NonZeroDuration::from_duration(duration.get() - difference)
                    else {
                        // The note was fully outside the clip on the left.
                        return Ok(None);
                    };

                    duration = max_duration;
                }

                let position = note_position.relative_to(clip_position);

                let note = Note::new(duration, self.id);

                let entry = HistoryEntry::InsertNote(note.id());

                let entry = self
                    .content
                    .as_notes_mut()
                    .ok_or(NoNotesSelected)?
                    .try_insert(position, pitch, note)
                    .map_or_else(|NoteInsertionError| None, |()| Some(entry));

                Ok(entry)
            }
        }
    }
}
