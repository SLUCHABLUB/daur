//! Items pertaining to [`Instant`].

mod ops;

use crate::audio::sample::Duration;

/// An instant in sample time. A sample index.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    /// The duration since the compositions start.
    pub since_start: Duration,
}

impl Instant {
    /// The starting point.
    pub const START: Instant = Instant {
        since_start: Duration::ZERO,
    };

    /// Converts the instant to a sample index.
    #[must_use]
    pub fn index(self) -> usize {
        self.since_start.samples
    }

    /// Constructs a new instant from a sample index.
    #[must_use]
    pub fn from_index(index: usize) -> Instant {
        Instant {
            since_start: Duration { samples: index },
        }
    }
}
