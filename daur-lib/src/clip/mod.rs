mod content;
mod overview;
mod source;

pub use content::ClipContent;
pub use overview::overview;
pub use source::ClipSource;

use crate::Colour;
use crate::time::{Instant, Mapping, Period};
use arcstr::ArcStr;

/// A clip inside a [`Track`]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Clip {
    /// The name of the clip
    pub name: ArcStr,
    /// The colour of the clip
    pub colour: Colour,
    /// The content of the clip
    pub content: ClipContent,
}

impl Clip {
    /// The [`Period`] of the clip
    #[must_use]
    pub fn period(&self, start: Instant, mapping: &Mapping) -> Period {
        self.content.period(start, mapping)
    }

    /// Returns a [`Source`](source::Source) for the clip
    pub fn to_source(&self, offset: usize) -> ClipSource {
        match &self.content {
            ClipContent::Audio(audio) => ClipSource::Audio(audio.to_source(offset)),
            ClipContent::Notes(_) => ClipSource::Notes,
        }
    }
}
