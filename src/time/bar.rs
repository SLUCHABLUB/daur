use crate::app::settings::OverviewSettings;
use crate::time::duration::Duration;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;
use saturating_cast::SaturatingCast;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Bar {
    pub start: Instant,
    pub time_signature: TimeSignature,
}

impl Bar {
    pub fn duration(&self) -> Duration {
        self.time_signature.bar_duration()
    }

    pub fn period(&self) -> Period {
        Period {
            start: self.start,
            duration: self.duration(),
        }
    }

    /// Return the number of grid cells that fit in the bar, rounded up
    pub fn grid_cell_count(&self, overview_settings: OverviewSettings) -> u64 {
        let exact = self.time_signature.bar_duration() / overview_settings.cell_duration;
        exact.ceil()
    }

    pub fn column_width(&self, overview_settings: OverviewSettings) -> u16 {
        self.grid_cell_count(overview_settings)
            .saturating_cast::<u16>()
            * overview_settings.cell_width
    }
}
