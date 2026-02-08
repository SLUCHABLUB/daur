//! Items pertaining to [`Quantisation`].

use crate::metre::NonZeroDuration;
use crate::ui::NonZeroLength;

/// Settings for quantisation.
///
/// The grid resets every measure, thus,
/// if the duration of a measure is not a multiple of [`cell_duration`](Quantisation::cell_duration),
/// the number of cells will be rounded upwards.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Quantisation {
    /// The duration of a full grid cell.
    pub cell_duration: NonZeroDuration,
    /// The width of a grid cell.
    pub cell_width: NonZeroLength,
}
