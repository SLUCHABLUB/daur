mod non_zero;
mod ops;

pub use non_zero::NonZeroDuration;

use crate::ratio::Ratio;
use serde::Deserialize;

/// A musical duration
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct Duration {
    /// The number of whole-note durations
    pub whole_notes: Ratio,
}

impl Duration {
    /// No time
    pub const ZERO: Duration = Duration {
        whole_notes: Ratio::ZERO,
    };
}
