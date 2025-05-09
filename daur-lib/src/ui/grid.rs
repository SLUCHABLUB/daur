use crate::musical_time::NonZeroDuration;
use crate::ui::NonZeroLength;

/// Settings for the overview/piano-roll grid.
///
/// The grid resets every bar, thus,
/// if the duration of a bar is not a multiple of [`cell_duration`](Grid::cell_duration),
/// the number of cells will be rounded upwards.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Grid {
    /// The duration of a full grid cell
    pub cell_duration: NonZeroDuration,
    /// The width of a grid cell
    pub cell_width: NonZeroLength,
}
