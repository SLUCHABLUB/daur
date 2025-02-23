use crate::project::changing::Changing;
use crate::time::{Bar, Instant, Signature};
use crate::ui::grid::Grid;
use crate::ui::{Length, Offset};
use std::sync::Arc;

/// A mapping between screen (x-)coordinates and musical time
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Mapping {
    /// The project's time signature
    pub time_signature: Arc<Changing<Signature>>,
    /// The grid settings
    pub grid: Grid,
    /// The offset from [`Instant::START`]
    pub offset: Length,
}

impl Mapping {
    /// Calculates the display-width of `bad`
    #[must_use]
    pub fn bar_width(&self, bar: Bar) -> Length {
        let cell_count = bar.period().duration / self.grid.cell_duration;

        self.grid.cell_width.get() * cell_count
    }

    fn absolute_offset(&self, instant: Instant) -> Length {
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

    /// Maps an [`Instant`] to an offset from the left of the window
    #[must_use]
    pub fn offset(&self, instant: Instant) -> Offset {
        Offset::from(self.absolute_offset(instant)) - self.offset
    }

    /// Maps an [`Instant`] to an offset from the left of the window if it is less than `max`
    #[must_use]
    pub fn offset_in_range(&self, instant: Instant, max: Length) -> Option<Length> {
        let offset = self.offset(instant).to_length()?;

        (offset < max).then_some(offset)
    }

    /// Maps offset from the left of the window to an [`Instant`] on the grid
    #[must_use]
    pub fn instant_on_grid(&self, offset: Length) -> Instant {
        let offset = self.offset + offset;

        let cell = (offset / self.grid.cell_width).floored();
        let duration = self.grid.cell_duration.get() * cell;
        Instant {
            since_start: duration,
        }
    }
}
