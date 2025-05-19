use crate::audio;
use crate::metre::{Instant, NonZeroPeriod};
use crate::notes::Notes;
use crate::project::Settings;
use crate::ui::{Grid, Length};
use crate::view::Context;

/// The content of a [clip](crate::track::Clip).
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Content {
    /// An audio clip.
    Audio(audio::NonEmpty),
    /// A notes clip.
    Notes(Notes),
    // TODO:
    //  - linked audio file
    //  - linked clip
    //  - drums
}

impl Content {
    /// Calculates the period of the content.
    #[must_use]
    pub fn period(&self, start: Instant, settings: &Settings) -> NonZeroPeriod {
        match self {
            Content::Audio(audio) => audio.period(start, settings),
            Content::Notes(notes) => NonZeroPeriod {
                start,
                duration: notes.duration(),
            },
        }
    }

    /// Tries to resolve the content to a notes-clip.
    #[must_use]
    pub fn as_notes(&self) -> Option<&Notes> {
        // TODO: also return notes if self is a linked clip
        match self {
            Content::Audio(_) => None,
            Content::Notes(notes) => Some(notes),
        }
    }

    /// Tries to resolve the content to a notes-clip.
    #[must_use]
    pub fn as_notes_mut(&mut self) -> Option<&mut Notes> {
        // TODO: also return notes if self is a linked clip
        match self {
            Content::Audio(_) => None,
            Content::Notes(notes) => Some(notes),
        }
    }

    pub(super) fn paint_overview(
        &self,
        context: &mut dyn Context,
        settings: &Settings,
        grid: Grid,
        crop_start: Length,
    ) {
        match self {
            Content::Audio(audio) => {
                audio.draw_overview(context, settings, grid, crop_start);
            }
            Content::Notes(notes) => notes.draw_overview(context),
        }
    }
}
