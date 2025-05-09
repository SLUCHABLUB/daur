use crate::musical_time::{Duration, Instant, Period, Signature};
use crate::project::Settings;
use crate::ui::{Grid, Length};

/// A bar
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Bar {
    /// When the bar starts
    pub start: Instant,
    /// The time signature of the bar
    pub time_signature: Signature,
}

impl Bar {
    /// The duration of the bar
    #[must_use]
    pub fn duration(self) -> Duration {
        self.time_signature.bar_duration().get()
    }

    /// The period of the bar
    #[must_use]
    pub fn period(self) -> Period {
        Period {
            start: self.start,
            duration: self.duration(),
        }
    }

    pub(crate) fn next(self, settings: &Settings) -> Bar {
        let start = self.period().end();
        Bar {
            start,
            time_signature: settings.time_signature.get(start),
        }
    }

    pub(crate) fn width(&self, grid: Grid) -> Length {
        grid.cell_width.get() * (self.duration() / grid.cell_duration).ceiled()
    }
}
