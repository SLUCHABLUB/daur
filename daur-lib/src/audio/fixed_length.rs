use crate::metre::{Instant, NonZeroDuration};
use crate::ui::{Grid, Length};
use crate::view::Painter;
use crate::{Audio, project, time};

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
        project_settings: &project::Settings,
    ) -> Option<FixedLength> {
        let duration = time::Period {
            start: position.to_real_time(project_settings),
            duration: audio.real_duration(),
        }
        .to_metre(project_settings)
        .duration;

        Some(FixedLength {
            audio,
            duration: NonZeroDuration::from_duration(duration)?,
        })
    }

    /// Draws an overview of the audio.
    pub(crate) fn overview_painter(
        &self,
        _project_settings: &project::Settings,
        _grid: Grid,
        _crop_start: Length,
    ) -> Box<Painter> {
        // TODO: draw loudness graph
        let _: &Self = self;
        Box::new(|_| ())
    }
}
