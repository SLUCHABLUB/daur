//! Items pertaining to [`Instant`].

mod non_zero;
mod ops;

pub use non_zero::NonZeroInstant;

use crate::metre::Duration;
use crate::metre::relative;
use serde::Deserialize;
use serde::Serialize;

/// An instant in musical time.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Instant {
    /// The duration since the starting point.
    pub since_start: Duration,
}

impl Instant {
    /// The starting point.
    pub const START: Instant = Instant {
        since_start: Duration::ZERO,
    };

    /// Returns this instant relative to another instant.
    pub(crate) fn relative_to(self, other: Instant) -> relative::Instant {
        relative::Instant {
            since_start: self - other,
        }
    }
}
