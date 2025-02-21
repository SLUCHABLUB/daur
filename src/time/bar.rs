use crate::app::OverviewSettings;
use crate::length::Length;
use crate::ratio::Ratio;
use crate::time::duration::Duration;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Bar {
    pub start: Instant,
    pub time_signature: TimeSignature,
}

impl Bar {
    pub fn column_width(&self, overview_settings: OverviewSettings) -> Length {
        overview_settings.cell_width * self.grid_cell_count(overview_settings)
    }

    // TODO: should we round up here?
    /// Return the number of grid cells that fit in the bar, rounded up
    pub fn grid_cell_count(&self, overview_settings: OverviewSettings) -> Ratio {
        let exact = self.time_signature.bar_duration() / overview_settings.cell_duration;
        exact.ceiled()
    }

    pub fn duration(&self) -> Duration {
        self.time_signature.bar_duration()
    }

    pub fn period(&self) -> Period {
        Period {
            start: self.start,
            duration: self.duration(),
        }
    }
}
