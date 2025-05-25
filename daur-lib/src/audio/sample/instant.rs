use crate::audio::sample::Duration;

/// An instant in sample time. A sample index.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    /// The duration since the compositions start.
    pub since_start: Duration,
}
