use crate::length::Length;
use crate::time::duration::Duration;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct OverviewSettings {
    /// The duration of a grid unit
    pub cell_duration: Duration,
    /// The number of columns per grid unit
    pub cell_width: Length,
    /// The offset in columns
    pub offset: Length,
}

impl Default for OverviewSettings {
    fn default() -> Self {
        OverviewSettings {
            cell_duration: Duration::QUARTER,
            cell_width: Length::DEFAULT_CELL_WIDTH,
            offset: Length::ZERO,
        }
    }
}
