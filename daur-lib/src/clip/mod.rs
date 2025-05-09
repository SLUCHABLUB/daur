//! Items pertaining to [`Clip`].

mod content;
mod overview;

pub use content::Content;
pub(crate) use overview::overview;

use crate::metre::{Instant, NonZeroPeriod};
use crate::project::Settings;
use crate::ui::Colour;
use arcstr::ArcStr;
use getset::CloneGetters;

/// A part of a [track](crate::Track).
#[doc(hidden)]
#[derive(Clone, Eq, PartialEq, Debug, CloneGetters)]
pub struct Clip {
    /// The name of the clip.
    #[get_clone = "pub"]
    pub name: ArcStr,
    /// The colour of the clip.
    pub colour: Colour,
    /// The content of the clip.
    pub content: Content,
}

impl Clip {
    /// Calculates the [period](NonZeroPeriod) of the clip.
    #[must_use]
    pub fn period(&self, start: Instant, settings: &Settings) -> NonZeroPeriod {
        self.content.period(start, settings)
    }
}
