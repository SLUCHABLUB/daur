use crate::metre::{Changing, Instant, NonZeroDuration, OffsetMapping, TimeContext};
use crate::ui::Length;
use crate::view::Painter;
use crate::{Audio, time};

// TODO: add a "reset size" context-menu item for recalculating the duration
/// Some audio that may be cropped or extended with silence to fit a duration.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FixedLength {
    /// The audio.
    pub audio: Audio,
    /// The length of the audio.
    pub duration: NonZeroDuration,
}

impl FixedLength {
    #[must_use]
    pub(crate) fn from_audio(
        audio: Audio,
        position: Instant,
        time_context: &Changing<TimeContext>,
    ) -> FixedLength {
        let duration = (time::Period {
            start: position * time_context,
            duration: audio.real_duration(),
        } / time_context)
            .duration;

        FixedLength {
            audio,
            duration: NonZeroDuration::from_duration(duration).unwrap_or(NonZeroDuration::QUARTER),
        }
    }

    /// Draws an overview of the audio.
    pub(crate) fn overview_painter(
        &self,
        _offset_mapping: OffsetMapping,
        _crop_start: Length,
    ) -> Box<Painter> {
        // TODO: draw loudness graph
        let _: &Self = self;
        Box::new(|_| ())
    }
}
