use crate::time::NonZeroDuration;
use crate::ui::{Length, NonZeroLength};

#[derive(Copy, Clone, Debug)]
pub struct OverviewSettings {
    /// The duration of a grid unit
    pub cell_duration: NonZeroDuration,
    /// The number of columns per grid unit
    pub cell_width: NonZeroLength,
    /// The offset in columns
    pub offset: Length,
}

impl Default for OverviewSettings {
    fn default() -> Self {
        OverviewSettings {
            cell_duration: NonZeroDuration::QUARTER,
            cell_width: NonZeroLength::DEFAULT_CELL_WIDTH,
            offset: Length::ZERO,
        }
    }
}
