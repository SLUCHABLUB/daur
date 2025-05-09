use crate::musical_time;
use crate::project::Settings;
use crate::real_time::{Duration, Instant};

/// A period of real time.
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

    /// Converts the period to musical time.
    #[must_use]
    pub fn to_metre(self, settings: &Settings) -> musical_time::Period {
        let start = self.start.to_metre(settings);
        let end = self.end().to_metre(settings);

        musical_time::Period {
            start,
            duration: end - start,
        }
    }
}
