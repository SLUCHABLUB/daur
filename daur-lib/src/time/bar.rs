use crate::app::OverviewSettings;
use crate::length::Length;
use crate::ratio::Ratio;
use crate::time::duration::Duration;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::signature::Signature;

/// A bar, or measure
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Bar {
    /// When the bar starts
    pub start: Instant,
    /// The time signature of the bar
    pub time_signature: Signature,
}

impl Bar {
    /// The display-width of the bar
    #[must_use]
    pub fn width(&self, overview_settings: OverviewSettings) -> Length {
        overview_settings.cell_width * self.grid_cell_count(overview_settings)
    }

    // TODO: should we round up here?
    /// Returns the number of grid cells that fit in the bar, rounded up
    #[must_use]
    pub fn grid_cell_count(&self, overview_settings: OverviewSettings) -> Ratio {
        let exact = self.time_signature.bar_duration() / overview_settings.cell_duration;
        exact.get().ceiled()
    }

    /// The duration of the bar
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.time_signature.bar_duration().get()
    }

    /// The period of the bar
    #[must_use]
    pub fn period(&self) -> Period {
        Period {
            start: self.start,
            duration: self.duration(),
        }
    }
}
