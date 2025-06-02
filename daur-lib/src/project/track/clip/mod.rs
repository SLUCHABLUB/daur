//! Items pertaining to [`Clip`].

mod content;
mod id;
mod overview;

pub use content::Content;
pub use id::Id;

pub(crate) use overview::overview;

use crate::audio::{FixedLength, sample};
use crate::metre::{Changing, Instant, NonZeroDuration, TimeContext};
use crate::note;
use crate::note::Event;
use crate::project::track;
use crate::ui::Colour;
use arcstr::{ArcStr, literal};
use getset::{CloneGetters, CopyGetters, Getters, MutGetters};
use sorted_vec::SortedVec;

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

/// A part of a [track](super::Track).
// TODO: Test that this isn't `Clone` (bc. id).
#[cfg_attr(doc, doc(hidden))]
#[derive(Debug, Getters, MutGetters, CopyGetters, CloneGetters, MutGetters)]
pub struct Clip {
    #[get_copy = "pub(crate)"]
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
    // TODO: derive
    pub(in crate::project) fn content_mut(&mut self) -> &mut Content {
        &mut self.content
    }

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
}
