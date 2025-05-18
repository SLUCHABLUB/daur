//! Items pertaining to [`Clip`].

mod content;
mod overview;

use arcstr::{ArcStr, literal};
pub use content::Content;
pub(crate) use overview::overview;

use crate::audio::NonEmpty;
use crate::metre::{Instant, NonZeroDuration, NonZeroPeriod};
use crate::ui::Colour;
use crate::{Notes, project};
use getset::{CloneGetters, CopyGetters, Getters, MutGetters};

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

/// A part of a [track](crate::Track).
#[doc(hidden)]
#[derive(Clone, Eq, PartialEq, Debug, Getters, MutGetters, CopyGetters, CloneGetters)]
pub struct Clip {
    /// The name of the clip.
    #[get_clone = "pub"]
    name: ArcStr,
    /// The colour of the clip.
    #[get_copy = "pub"]
    colour: Colour,
    /// The content of the clip.
    #[get = "pub(crate)"]
    #[get_mut = "pub(crate)"]
    content: Content,
}

impl Clip {
    #[must_use]
    pub fn from_audio(name: ArcStr, audio: NonEmpty) -> Clip {
        Clip {
            name,
            colour: DEFAULT_AUDIO_COLOUR,
            content: Content::Audio(audio),
        }
    }

    #[must_use]
    pub fn empty_notes(duration: NonZeroDuration) -> Clip {
        Clip {
            name: DEFAULT_NOTES_NAME,
            colour: DEFAULT_NOTES_COLOUR,
            content: Content::Notes(Notes::empty(duration)),
        }
    }

    /// Calculates the [period](NonZeroPeriod) of the clip.
    #[must_use]
    pub fn period(&self, start: Instant, settings: &project::Settings) -> NonZeroPeriod {
        self.content.period(start, settings)
    }
}
