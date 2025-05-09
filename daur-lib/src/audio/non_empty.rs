use crate::Audio;
use crate::metre::{Instant, NonZeroPeriod, Period};
use crate::project::Settings;
use crate::ui::Grid;
use crate::view::Context;

/// Some audio of positive length.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NonEmpty {
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
    pub(crate) fn period(&self, start: Instant, settings: &Settings) -> NonZeroPeriod {
        // TODO: do this more cleanly
        NonZeroPeriod::from_period(self.inner.period(start, settings)).unwrap_or_else(|| {
            // This should be unreachable
            let duration = settings.time_signature.get(start).beat_duration();
            NonZeroPeriod { start, duration }
        })
    }

    /// Draws an overview of the audio.
    pub(crate) fn draw_overview(
        &self,
        context: &mut dyn Context,
        full_period: Period,
        visible_period: Period,
        settings: &Settings,
        grid: Grid,
    ) {
        // TODO: draw loudness graph
        let _ = (self, context, full_period, visible_period, settings, grid);
    }
}
