use crate::audio::Audio;
use crate::notes::Notes;
use crate::time::{Instant, Mapping, Period};
use crate::view::Context;

/// The content of a [`Clip`](crate::Clip)
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ClipContent {
    /// An audio clip
    Audio(Audio),
    /// A notes clip
    Notes(Notes),
    // TODO:
    //  - linked audio file
    //  - linked clip
    //  - drums
}

impl ClipContent {
    /// Returns the period of the content
    #[must_use]
    pub fn period(&self, start: Instant, mapping: &Mapping) -> Period {
        match self {
            ClipContent::Audio(audio) => audio.period(start, mapping),
            ClipContent::Notes(notes) => Period {
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
            ClipContent::Audio(audio) => {
                audio.draw_overview(context, full_period, visible_period, mapping);
            }
            ClipContent::Notes(notes) => notes.draw_overview(context),
        }
    }
}
