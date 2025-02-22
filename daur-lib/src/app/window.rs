use crate::app::settings::OverviewSettings;
use crate::measure::{Length, Offset, Rectangle};
use crate::project::changing::Changing;
use crate::time::{Instant, Period, Signature};
use std::sync::Arc;

/// A window into
#[derive(Clone)]
pub struct Window {
    pub time_signature: Arc<Changing<Signature>>,
    pub overview_settings: OverviewSettings,
    pub x: Length,
    pub width: Length,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct UncheckedRect {
    pub x: Offset,
    pub y: Offset,
    pub width: Length,
    pub height: Length,
}

impl UncheckedRect {
    pub fn clamp(self) -> Rectangle {
        Rectangle {
            x: self.x.saturate(),
            y: self.y.saturate(),
            width: self.width,
            height: self.height,
        }
    }
}

impl Window {
    pub fn column_to_instant_on_grid(&self, column: Length) -> Instant {
        let offset = self.overview_settings.offset + column - self.x;

        let cell = (offset / self.overview_settings.cell_width).rounded();
        let duration = self.overview_settings.cell_duration.get() * cell;
        Instant {
            since_start: duration,
        }
    }

    pub fn instant_to_column(&self, instant: Instant) -> Option<Length> {
        let column_unchecked = self.instant_to_column_unchecked(instant);
        let column = column_unchecked.to_length()?;

        if column < self.width {
            Some(column + self.x)
        } else {
            None
        }
    }

    fn instant_to_column_unchecked(&self, instant: Instant) -> Offset {
        let mut column = Offset::ZERO;

        for bar in self.time_signature.bars() {
            if !bar.period().contains(instant) {
                let width = bar.width(self.overview_settings);
                column += width;
                continue;
            }

            let offset = instant - bar.start;

            let cell_offset = offset / self.overview_settings.cell_duration;

            column += self.overview_settings.cell_width.get() * cell_offset;

            break;
        }

        column - self.overview_settings.offset
    }

    pub fn period_to_unchecked_rect(
        &self,
        period: Period,
        x: Length,
        y: Length,
        height: Length,
    ) -> UncheckedRect {
        let start = self.instant_to_column_unchecked(period.start) + x;
        // TODO: subtract one to not include column of next beat?
        let end = self.instant_to_column_unchecked(period.end()) + x;
        let width = (end - start).saturate();

        UncheckedRect {
            x: start,
            y: Offset::from(y),
            width,
            height,
        }
    }
}
