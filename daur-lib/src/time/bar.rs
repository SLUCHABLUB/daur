use crate::time::{Duration, Instant, Period, Signature};

/// A bar, or ui
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Bar {
    /// When the bar starts
    pub start: Instant,
    /// The time signature of the bar
    pub time_signature: Signature,
}

impl Bar {
    /// The duration of the bar
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.time_signature.bar_duration().get()
    }

    /// The period of the bar
    #[must_use]
    pub fn period(&self) -> Period {
        Period {
            start: self.start,
            duration: self.duration(),
        }
    }
}
