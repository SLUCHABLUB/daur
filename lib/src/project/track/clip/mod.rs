//! Items pertaining to [`Clip`].

mod content;
mod overview;
mod path;

pub use content::Content;
pub use path::Path;

pub(crate) use overview::overview;

use crate::Id;
use crate::audio::FixedLength;
use crate::audio::sample;
use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::NonZeroDuration;
use crate::metre::TimeContext;
use crate::note;
use crate::note::event::Sequence;
use crate::ui::Colour;
use arcstr::ArcStr;
use arcstr::literal;
use getset::CloneGetters;
use getset::CopyGetters;
use getset::Getters;
use getset::MutGetters;

const DEFAULT_AUDIO_COLOUR: Colour = Colour::LIME;

const DEFAULT_NOTES_NAME: ArcStr = literal!("some notes");
const DEFAULT_NOTES_COLOUR: Colour = Colour::MAGENTA;

/// A part of a [track](super::Track).
// TODO: Test that this isn't `Clone` (bc. id).
#[derive(Debug, Getters, MutGetters, CopyGetters, CloneGetters, MutGetters)]
pub struct Clip {
    #[get_copy = "pub(crate)"]
    id: Id<Clip>,
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
    pub(crate) fn from_audio(name: ArcStr, audio: FixedLength) -> Clip {
        Clip {
            id: Id::generate(),
            name,
            colour: DEFAULT_AUDIO_COLOUR,
            content: Content::Audio(audio),
        }
    }

    #[must_use]
    pub(crate) fn empty_notes(duration: NonZeroDuration) -> Clip {
        Clip {
            id: Id::generate(),
            name: DEFAULT_NOTES_NAME,
            colour: DEFAULT_NOTES_COLOUR,
            content: Content::Notes(note::Group::empty(duration)),
        }
    }

    pub(crate) fn duration(&self) -> NonZeroDuration {
        self.content.duration()
    }

    pub(super) fn events(
        &self,
        clip_start: Instant,
        time_context: &Changing<TimeContext>,
        sample_rate: sample::Rate,
    ) -> Sequence {
        let Some(notes) = self.content.as_notes() else {
            return Sequence::new();
        };

        notes.to_events(clip_start, time_context, sample_rate)
    }
}
