use crate::app::settings::OverviewSettings;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;
use crate::time::Ratio;
use ratatui::layout::Rect;
use rounded_div::RoundedDiv;
use saturating_cast::SaturatingCast;

/// A window into
#[derive(Copy, Clone)]
pub struct Window<'a> {
    pub time_signature: &'a Changing<TimeSignature>,
    pub overview_settings: OverviewSettings,
    pub x: u16,
    pub width: u16,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct UncheckedRect {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
}

impl UncheckedRect {
    pub fn clamp(self) -> Rect {
        Rect {
            x: self.x.saturating_cast(),
            y: self.y.saturating_cast(),
            width: self.width,
            height: self.height,
        }
    }
}

impl Window<'_> {
    fn instant_to_column_unchecked(self, instant: Instant) -> i32 {
        let mut column = 0;

        for bar in self.time_signature.bars() {
            if !bar.period().contains(instant) {
                let width = bar.column_width(self.overview_settings);
                column += i32::from(width);
                continue;
            }

            let offset = instant - bar.start;

            let cell_offset = offset / self.overview_settings.cell_duration;

            column += (cell_offset * Ratio::from(self.overview_settings.cell_width))
                .saturating_cast::<i32>();

            break;
        }

        column - i32::from(self.overview_settings.offset)
    }

    pub fn instant_to_column(self, instant: Instant) -> Option<u16> {
        let column_unchecked = self.instant_to_column_unchecked(instant);
        let column = u16::try_from(column_unchecked).ok()?;

        if column < self.width {
            Some(column + self.x)
        } else {
            None
        }
    }

    pub fn column_to_instant_on_grid(&self, column: u16) -> Instant {
        let offset = self.overview_settings.offset + column - self.x;

        let cell = offset.rounded_div(self.overview_settings.cell_width);
        let duration = self.overview_settings.cell_duration * Ratio::from(cell);
        Instant {
            whole_notes: duration.whole_notes,
        }
    }

    pub fn period_to_unchecked_rect(
        self,
        period: Period,
        x: u16,
        y: u16,
        height: u16,
    ) -> UncheckedRect {
        let offset = i32::from(x);
        let x = offset + self.instant_to_column_unchecked(period.start);
        // TODO: subtract one to not include column of next beat?
        let end = offset + self.instant_to_column_unchecked(period.end());
        let width = (end - x).saturating_cast();

        UncheckedRect {
            x,
            y: i32::from(y),
            width,
            height,
        }
    }
}
