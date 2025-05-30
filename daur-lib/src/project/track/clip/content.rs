use crate::metre::NonZeroDuration;
use crate::ui::{Grid, Length};
use crate::view::Painter;
use crate::{audio, note, project};

/// The content of a [clip](super::Clip).
#[derive(Eq, PartialEq, Debug)]
pub enum Content {
    /// An audio clip.
    Audio(audio::FixedLength),
    /// A notes clip.
    Notes(note::Group),
    // TODO:
    //  - linked audio file
    //  - linked clip
    //  - drums
}

impl Content {
    pub(crate) fn duration(&self) -> NonZeroDuration {
        match self {
            Content::Audio(audio) => audio.duration,
            Content::Notes(notes) => notes.duration(),
        }
    }

    /// Tries to resolve the content to a notes-clip.
    #[must_use]
    pub fn as_audio(&self) -> Option<&audio::FixedLength> {
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

    pub(super) fn overview_painter(
        &self,
        project_settings: &project::Settings,
        grid: Grid,
        crop_start: Length,
    ) -> Box<Painter> {
        match self {
            Content::Audio(audio) => audio.overview_painter(project_settings, grid, crop_start),
            Content::Notes(notes) => notes.overview_painter(),
        }
    }
}
