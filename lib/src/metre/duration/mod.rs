//! Items pertaining to [`Duration`].

mod non_zero;
mod ops;

pub use non_zero::NonZeroDuration;

use crate::Ratio;
use serde::Deserialize;
use serde::Serialize;

/// A musical duration.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Duration {
    /// The number of whole-note durations.
    pub whole_notes: Ratio,
}

impl Duration {
    /// No time.
    pub const ZERO: Duration = Duration {
        whole_notes: Ratio::ZERO,
    };
}
