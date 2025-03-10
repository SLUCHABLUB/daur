use crate::project::changing::Changing;
use crate::time::{Bar, Instant, Period, Signature};
use crate::ui::{Grid, Length};
use std::sync::Arc;

/// A mapping between screen (x-)coordinates and musical time
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Mapping {
    /// The project's time signature
    pub time_signature: Arc<Changing<Signature>>,
    /// The grid settings
    pub grid: Grid,
}

impl Mapping {
    /// Calculates the display-width of `bar`
    #[must_use]
    pub fn bar_width(&self, bar: Bar) -> Length {
        let cell_count = bar.period().duration / self.grid.cell_duration;

        self.grid.cell_width.get() * cell_count
    }

    /// Maps an [`Instant`] to an offset from the left of the window
    #[must_use]
    pub fn offset(&self, instant: Instant) -> Length {
        let mut offset = Length::ZERO;

        for bar in self.time_signature.bars() {
            if !bar.period().contains(instant) {
                offset += self.bar_width(bar);
                continue;
            }

            let remaining = instant - bar.start;

            let cell_count = remaining / self.grid.cell_duration;

            offset += self.grid.cell_width.get() * cell_count;

            break;
        }

        offset
    }

    /// Maps an offset from the left of the window to an [`Instant`] on the grid
    #[must_use]
    pub fn instant_on_grid(&self, offset: Length) -> Instant {
        let cell = (offset / self.grid.cell_width).floored();
        let duration = self.grid.cell_duration.get() * cell;
        Instant {
            since_start: duration,
        }
    }

    /// Maps an offset from the left of the window to an [`Instant`]
    #[must_use]
    pub fn instant(&self, offset: Length) -> Instant {
        let cell = offset / self.grid.cell_width;
        let duration = self.grid.cell_duration.get() * cell;
        Instant {
            since_start: duration,
        }
    }

    /// Maps an offset from the left of the window and a width to a [`Period`]
    #[must_use]
    pub fn period(&self, x: Length, width: Length) -> Period {
        let end = x + width;
        let start = self.instant(x);
        let end = self.instant(end);
        let duration = end - start;
        Period { start, duration }
    }

    /// Calculates the width of a `Period`
    #[must_use]
    pub fn width_of(&self, period: Period) -> Length {
        let start = self.offset(period.start);
        let end = self.offset(period.end());

        end - start
    }
}
