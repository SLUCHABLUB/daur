use crate::metre::{Instant, NonZeroPeriod};
use crate::ui::{Grid, Length};
use crate::view::Context;
use crate::{Audio, project};

/// Some audio of positive length.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NonEmpty {
    // TODO: use Vec1
    inner: Audio,
}

impl NonEmpty {
    /// Tries to construct a [`NonEmpty`] from an [`Audio`].
    #[must_use]
    pub fn from_audio(audio: Audio) -> Option<NonEmpty> {
        if audio.samples.is_empty() {
            return None;
        }

        Some(NonEmpty { inner: audio })
    }

    /// Converts a reference to the audio to an [`Audio`] reference.
    #[must_use]
    pub fn as_audio(&self) -> &Audio {
        &self.inner
    }

    /// Converts the audio to an [`Audio`].
    #[must_use]
    pub fn into_audio(self) -> Audio {
        self.inner
    }

    /// Returns the period of the audio.
    #[must_use]
    pub(crate) fn period(
        &self,
        start: Instant,
        project_settings: &project::Settings,
    ) -> NonZeroPeriod {
        // TODO: do this more cleanly
        NonZeroPeriod::from_period(self.inner.period(start, project_settings)).unwrap_or_else(
            || {
                // This should be unreachable
                let duration = project_settings.time_signature.get(start).beat_duration();
                NonZeroPeriod { start, duration }
            },
        )
    }

    /// Draws an overview of the audio.
    pub(crate) fn draw_overview(
        &self,
        _context: &mut dyn Context,
        _project_settings: &project::Settings,
        _grid: Grid,
        _crop_start: Length,
    ) {
        // TODO: draw loudness graph
        let _: &Self = self;
    }
}
