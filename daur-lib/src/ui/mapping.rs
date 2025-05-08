use crate::Changing;
use crate::musical_time::{Bar, Instant, Period, Signature};
use crate::ui::{Grid, Length};
use std::sync::Arc;

/// A mapping between screen (x-)coordinates and musical time.
///
/// Any horizontal display of musical time is divided into cells.
/// The size of a cell is fixed and specified in the [grid settings](Grid).
/// If the duration of a bar is not a multiple of the cell duration,
/// the bar will be displayed as if it was a little longer.
/// More precisely, the ["cell count" of a bar](Mapping::bar_cell_count) is rounded up.
///
/// Using the time signature,
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Mapping {
    /// The project's time signature.
    pub time_signature: Arc<Changing<Signature>>,
    /// The grid settings.
    pub grid: Grid,
}

impl Mapping {
    // TODO: maybe make the last cell smaller if possible?
    /// Returns the numer of grid-cells that take up the bar (rounded up).
    #[must_use]
    pub fn bar_cell_count(&self, bar: Bar) -> u64 {
        (bar.period().duration / self.grid.cell_duration).ceil()
    }

    /// Calculates the display-width of a bar.
    #[must_use]
    pub fn bar_width(&self, bar: Bar) -> Length {
        self.grid.cell_width.get() * self.bar_cell_count(bar).into()
    }

    /// Maps an [instant](Instant) to an offset from the left of the window.
    #[must_use]
    pub fn x_offset(&self, instant: Instant) -> Length {
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

    /// Maps an offset from the left of the window to an [instant](Instant) on the grid (by rounding).
    #[must_use]
    pub fn instant_on_grid(&self, offset: Length) -> Instant {
        let cell = (offset / self.grid.cell_width).rounded();
        let duration = self.grid.cell_duration.get() * cell;
        Instant {
            since_start: duration,
        }
    }

    /// Maps an offset from the left of the window to an [instant](Instant).
    #[must_use]
    pub fn instant(&self, offset: Length) -> Instant {
        let cell = offset / self.grid.cell_width;
        let duration = self.grid.cell_duration.get() * cell;
        Instant {
            since_start: duration,
        }
    }

    /// Maps an offset from the left of the window and a width to a [period](Period).
    #[must_use]
    pub fn period(&self, x: Length, width: Length) -> Period {
        let end = x + width;
        let start = self.instant(x);
        let end = self.instant(end);
        let duration = end - start;
        Period { start, duration }
    }

    /// Calculates the width of a period.
    ///
    /// For a `period: Period`,
    /// ```ignore
    /// self.x_offset(period.start) + self.width_of(period) == self.x_offset(period.end)
    /// ```
    #[must_use]
    pub fn width_of(&self, period: Period) -> Length {
        let start = self.x_offset(period.start);
        let end = self.x_offset(period.end());

        end - start
    }
}
