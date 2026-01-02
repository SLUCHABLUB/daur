use crate::NonZeroRatio;
use crate::metre::Duration;
use serde::Deserialize;
use serde::Serialize;

/// A non-zero duration.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct NonZeroDuration {
    /// The number of whole-note durations
    pub whole_notes: NonZeroRatio,
}

impl NonZeroDuration {
    /// The duration of a quarter note.
    pub const QUARTER: NonZeroDuration = NonZeroDuration {
        whole_notes: NonZeroRatio::QUARTER,
    };

    /// Converts the duration to a [zeroable one](Duration).
    #[must_use]
    pub fn get(self) -> Duration {
        Duration {
            whole_notes: self.whole_notes.get(),
        }
    }

    /// Converts a duration to a non-zero one if it is not zero.
    #[must_use]
    pub fn from_duration(duration: Duration) -> Option<NonZeroDuration> {
        Some(NonZeroDuration {
            whole_notes: NonZeroRatio::from_ratio(duration.whole_notes)?,
        })
    }
}

impl From<NonZeroDuration> for Duration {
    fn from(value: NonZeroDuration) -> Self {
        value.get()
    }
}
