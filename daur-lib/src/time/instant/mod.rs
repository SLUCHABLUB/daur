mod non_zero;

pub use non_zero::NonZeroInstant;

use crate::audio::sample;
use crate::time::Duration;
use crate::{metre, project};
use std::ops::{Add, AddAssign, Mul, Sub};

/// An instant in real time.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    /// The duration since the compositions start.
    pub since_start: Duration,
}

impl Instant {
    /// The starting point.
    pub const START: Instant = Instant {
        since_start: Duration::ZERO,
    };

    pub(crate) fn to_metre(self, project_settings: &project::Settings) -> metre::Instant {
        let mut remaining = self.since_start;
        let mut instant = metre::Instant::START;

        let mut change = metre::Instant::START;
        let mut tempo = project_settings.tempo.start;
        let mut time_signature = project_settings.time_signature.start;

        for (next_change, next_tempo, next_time_signature) in project_settings.time_changes() {
            let duration = next_change.get() - change;
            let real_duration =
                tempo.beat_duration().get() * (duration / time_signature.beat_duration());

            if remaining < real_duration {
                break;
            }

            instant += duration;
            remaining -= real_duration;

            change = next_change.get();
            tempo = next_tempo;
            time_signature = next_time_signature;
        }

        instant += time_signature.beat_duration().get() * (remaining / tempo.beat_duration());

        instant
    }
}

// TODO: derive
impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(mut self, rhs: Duration) -> Instant {
        self += rhs;
        self
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.since_start += rhs;
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Duration {
        self.since_start - rhs.since_start
    }
}

impl Mul<sample::Rate> for Instant {
    type Output = sample::Instant;

    fn mul(self, rhs: sample::Rate) -> sample::Instant {
        sample::Instant {
            since_start: self.since_start * rhs,
        }
    }
}
