//! Items pertaining to [`Clip`].

mod content;
mod overview;
mod path;
mod serial;

pub use content::Content;
pub use path::Path;

pub(in crate::project) use overview::overview;
pub(in crate::project) use serial::Serial;

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

/// The default colour for audio clips.
const DEFAULT_AUDIO_COLOUR: Colour = Colour::LIME;

/// The default name for note-group clips.
const DEFAULT_NOTES_NAME: ArcStr = literal!("some notes");
/// The default colour for note-group clips.
const DEFAULT_NOTES_COLOUR: Colour = Colour::MAGENTA;

/// A part of a [track](super::Track).
// TODO: Test that this isn't `Clone` (bc. id).
#[derive(Debug, Getters, MutGetters, CopyGetters, CloneGetters, MutGetters)]
pub struct Clip {
    /// The id.
    #[get_copy = "pub(crate)"]
    id: Id<Clip>,
    /// The name.
    #[get_clone = "pub"]
    name: ArcStr,

    /// The colour.
    #[get_copy = "pub"]
    colour: Colour,

    /// The content.
    #[get = "pub(crate)"]
    content: Content,
}

impl Clip {
    // TODO: derive
    /// Returns a mutable reference to the content.
    pub(in crate::project) fn content_mut(&mut self) -> &mut Content {
        &mut self.content
    }

    /// Constructs a new clip with a generated id.
    fn new(name: ArcStr, colour: Colour, content: Content) -> Clip {
        Clip {
            id: Id::generate(),
            name,
            colour,
            content,
        }
    }

    /// Constructs a new audio clip.
    #[must_use]
    pub(crate) fn from_audio(name: ArcStr, audio: FixedLength) -> Clip {
        Clip::new(name, DEFAULT_AUDIO_COLOUR, Content::Audio(audio))
    }

    /// Constructs an empty note-group clip.
    #[must_use]
    pub(crate) fn empty_notes(duration: NonZeroDuration) -> Clip {
        Clip::new(
            DEFAULT_NOTES_NAME,
            DEFAULT_NOTES_COLOUR,
            Content::Notes(note::Group::empty(duration)),
        )
    }

    /// Returns the duration of the clip.
    pub(crate) fn duration(&self) -> NonZeroDuration {
        self.content.duration()
    }

    /// Returns the [events](crate::note::Event) in the clip.
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
