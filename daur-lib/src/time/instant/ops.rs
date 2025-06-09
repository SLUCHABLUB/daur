use crate::time::{Duration, Instant};
use std::ops::{Add, AddAssign, Sub, SubAssign};

// --- INFIX OPERATIONS ---

impl<D: Into<Duration>> Add<D> for Instant {
    type Output = Instant;

    fn add(mut self, rhs: D) -> Instant {
        self += rhs;
        self
    }
}

impl<D: Into<Duration>> Sub<D> for Instant {
    type Output = Instant;

    fn sub(mut self, rhs: D) -> Instant {
        self -= rhs;
        self
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl<D: Into<Duration>> AddAssign<D> for Instant {
    fn add_assign(&mut self, rhs: D) {
        self.since_start += rhs;
    }
}

impl<D: Into<Duration>> SubAssign<D> for Instant {
    fn sub_assign(&mut self, rhs: D) {
        self.since_start -= rhs;
    }
}

// --- NON-CLOSED OPERATIONS ---

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.since_start - rhs.since_start
    }
}
