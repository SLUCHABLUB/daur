//! Items pertaining to [`Clip`].

mod content;
mod overview;
mod source;

pub use content::Content;
pub use overview::overview;
pub use source::Source;

use crate::Colour;
use crate::time::{Instant, Mapping, Period};
use arcstr::ArcStr;
use getset::CloneGetters;

/// A part of a [track](crate::Track).
#[doc(hidden)]
#[derive(Clone, Eq, PartialEq, Debug, CloneGetters)]
pub struct Clip {
    /// The name of the clip
    #[get_clone = "pub"]
    pub name: ArcStr,
    /// The colour of the clip
    pub colour: Colour,
    /// The content of the clip
    pub content: Content,
}

impl Clip {
    /// Calculates the [period](Period) of the clip.
    #[must_use]
    pub fn period(&self, start: Instant, mapping: &Mapping) -> Period {
        self.content.period(start, mapping)
    }

    /// Returns an [audio source](rodio::Source) for the clip.
    pub fn to_source(&self, offset: usize) -> Source {
        match &self.content {
            Content::Audio(audio) => Source::Audio(audio.to_source(offset)),
            Content::Notes(_) => Source::Notes,
        }
    }
}
