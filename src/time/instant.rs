use crate::time::duration::Duration;
use crate::time::Ratio;
use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    pub whole_notes: Ratio,
}

impl Instant {
    pub const START: Instant = Instant {
        whole_notes: Ratio::ZERO,
    };
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(mut self, rhs: Duration) -> Instant {
        self += rhs;
        self
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.whole_notes += rhs.whole_notes;
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        Duration {
            whole_notes: self.whole_notes - rhs.whole_notes,
        }
    }
}
