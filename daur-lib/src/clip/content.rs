use crate::audio;
use crate::musical_time::{Instant, NonZeroPeriod, Period};
use crate::notes::Notes;
use crate::project::Settings;
use crate::ui::Grid;
use crate::view::Context;

/// The content of a [clip](crate::Clip).
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

    pub(super) fn paint_overview(
        &self,
        context: &mut dyn Context,
        full_period: Period,
        visible_period: Period,
        settings: &Settings,
        grid: Grid,
    ) {
        match self {
            Content::Audio(audio) => {
                audio.draw_overview(context, full_period, visible_period, settings, grid);
            }
            Content::Notes(notes) => notes.draw_overview(context),
        }
    }
}
