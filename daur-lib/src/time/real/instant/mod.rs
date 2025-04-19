use crate::time::real::duration::NonZeroDuration;

mod non_zero;

pub use non_zero::NonZeroInstant;

/// An instant in real time.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Instant {
    /// The duration since the compositions start.
    pub since_start: NonZeroDuration,
}
