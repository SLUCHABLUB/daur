//! Items pertaining to [`Instant`].

mod non_zero;
mod ops;

pub use non_zero::NonZeroInstant;

use crate::audio::sample;
use crate::time::Duration;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Mul;

/// An instant in real time.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Instant {
    /// The duration since the compositions start.
    pub since_start: Duration,
}

impl Instant {
    /// The starting point.
    pub const START: Instant = Instant {
        since_start: Duration::ZERO,
    };
}

impl Mul<sample::Rate> for Instant {
    type Output = sample::Instant;

    fn mul(self, rhs: sample::Rate) -> sample::Instant {
        sample::Instant {
            since_start: self.since_start * rhs,
        }
    }
}
