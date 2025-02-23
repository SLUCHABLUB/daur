use crate::time::NonZeroDuration;
use crate::ui::NonZeroLength;

/// Settings for the overview/piano-roll grid
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Grid {
    /// The duration of a full grid cell
    pub cell_duration: NonZeroDuration,
    /// The width of a grid cell
    pub cell_width: NonZeroLength,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            cell_duration: NonZeroDuration::QUARTER,
            cell_width: NonZeroLength::DEFAULT_CELL_WIDTH,
        }
    }
}
