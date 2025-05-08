use crate::real_time::NonZeroDuration;

/// An [instant](super::Instant) that is strictly after the starting point.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonZeroInstant {
    /// The duration since the starting point
    pub since_start: NonZeroDuration,
}
