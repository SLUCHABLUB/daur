use crate::audio::Audio;
use crate::musical_time::{Instant, Mapping, Period};
use crate::notes::Notes;
use crate::view::Context;

/// The content of a [clip](crate::Clip).
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Content {
    /// An audio clip
    Audio(Audio),
    /// A notes clip
    Notes(Notes),
    // TODO:
    //  - linked audio file
    //  - linked clip
    //  - drums
}

impl Content {
    /// Calculates the period of the content.
    #[must_use]
    pub fn period(&self, start: Instant, mapping: &Mapping) -> Period {
        match self {
            Content::Audio(audio) => audio.period(start, mapping),
            Content::Notes(notes) => Period {
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
        mapping: &Mapping,
    ) {
        match self {
            Content::Audio(audio) => {
                audio.draw_overview(context, full_period, visible_period, mapping);
            }
            Content::Notes(notes) => notes.draw_overview(context),
        }
    }
}
