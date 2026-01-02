use crate::audio::sample::Duration;
use crate::audio::sample::Instant;
use std::ops::Range;

/// A period of sample time.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Period {
    /// The start of the period.
    pub start: Instant,
    /// The duration of the period.
    pub duration: Duration,
}

impl Period {
    /// Returns the end of the period.
    #[must_use]
    pub fn end(&self) -> Instant {
        self.start + self.duration
    }

    /// Return whether the period contains an instant.
    #[must_use]
    pub fn contains(self, instant: Instant) -> bool {
        self.range().contains(&instant.index())
    }

    /// Returns the range of sample indexes contained within the period.
    #[must_use]
    pub fn range(self) -> Range<usize> {
        self.start.index()..self.end().index()
    }
}
