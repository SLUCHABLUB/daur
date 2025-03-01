use crate::time::{Duration, Instant};
use std::ops::Range;

/// A period of musical time
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Period {
    /// Ths start of the period
    pub start: Instant,
    /// The duration of the period
    pub duration: Duration,
}

impl Period {
    /// The instant representing the end of the period
    #[must_use]
    pub fn end(&self) -> Instant {
        self.start + self.duration
    }

    fn range(self) -> Range<Instant> {
        Range {
            start: self.start,
            end: self.end(),
        }
    }

    /// Whether the period contains the specified instant
    #[must_use]
    pub fn contains(self, instant: Instant) -> bool {
        self.range().contains(&instant)
    }
}
