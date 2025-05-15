//! Items pertaining to [`Clip`].

mod content;
mod overview;
mod settings;

pub use content::Content;
pub(crate) use overview::overview;
pub use settings::Settings;

use crate::metre::{Instant, NonZeroPeriod};
use crate::project;
use getset::CloneGetters;

/// A part of a [track](crate::Track).
#[doc(hidden)]
#[derive(Clone, Eq, PartialEq, Debug, CloneGetters)]
pub struct Clip {
    pub settings: Settings,
    /// The content of the clip.
    pub content: Content,
}

impl Clip {
    /// Calculates the [period](NonZeroPeriod) of the clip.
    #[must_use]
    pub fn period(&self, start: Instant, settings: &project::Settings) -> NonZeroPeriod {
        self.content.period(start, settings)
    }
}
