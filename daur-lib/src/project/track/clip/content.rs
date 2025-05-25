use crate::metre::{Instant, NonZeroPeriod};
use crate::project::Settings;
use crate::ui::{Grid, Length};
use crate::view::Context;
use crate::{audio, note};

/// The content of a [clip](super::Clip).
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Content {
    /// An audio clip.
    Audio(audio::NonEmpty),
    /// A notes clip.
    Notes(note::Group),
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
    pub fn as_audio(&self) -> Option<&audio::NonEmpty> {
        match self {
            Content::Audio(audio) => Some(audio),
            Content::Notes(_) => None,
        }
    }

    /// Tries to resolve the content to a note group.
    #[must_use]
    pub fn as_notes(&self) -> Option<&note::Group> {
        match self {
            Content::Audio(_) => None,
            Content::Notes(notes) => Some(notes),
        }
    }

    /// Tries to resolve the content to a notes-clip.
    #[must_use]
    pub fn as_notes_mut(&mut self) -> Option<&mut note::Group> {
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
