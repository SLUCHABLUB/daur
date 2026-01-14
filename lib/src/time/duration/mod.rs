//! Items pertaining to [`Duration`].

mod non_zero;
mod ops;

use crate::Ratio;
use crate::audio::sample;
pub use non_zero::NonZeroDuration;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Mul;
use std::time;

/// A duration of real time.
///
/// Like [`std::time::Duration`](time::Duration), it can only represent time down to nanoseconds.
/// Furthermore, the maximum duration which is representable is only about 500 years.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Duration {
    /// The number of nanoseconds that the duration takes up.
    pub nanoseconds: u64,
}

impl Duration {
    /// 0
    pub const ZERO: Duration = Duration { nanoseconds: 0 };

    /// One nanosecond.
    pub const NANOSECOND: Duration = Duration { nanoseconds: 1 };

    /// 9 192 631 770 times the unperturbed ground-state hyperfine transition period of caesium-133.
    pub const SECOND: Duration = Duration {
        nanoseconds: 1_000_000_000,
    };

    /// One minute.
    pub const MINUTE: Duration = Duration {
        nanoseconds: 60_000_000_000,
    };
}

impl From<time::Duration> for Duration {
    fn from(duration: time::Duration) -> Duration {
        let nanoseconds = u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX);

        Duration { nanoseconds }
    }
}

impl From<Duration> for time::Duration {
    fn from(duration: Duration) -> time::Duration {
        time::Duration::from_nanos(duration.nanoseconds)
    }
}

impl Mul<sample::Rate> for Duration {
    type Output = sample::Duration;

    fn mul(self, rhs: sample::Rate) -> sample::Duration {
        let seconds = self / NonZeroDuration::SECOND;

        sample::Duration {
            samples: (seconds * Ratio::from(rhs.samples_per_second.get())).to_usize(),
        }
    }
}
