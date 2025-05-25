use crate::time::{Duration, Instant};
use crate::{metre, project};

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
    pub fn to_metre(self, project_settings: &project::Settings) -> metre::Period {
        let start = self.start.to_metre(project_settings);
        let end = self.end().to_metre(project_settings);

        metre::Period {
            start,
            duration: end - start,
        }
    }
}
