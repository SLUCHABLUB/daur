use crate::time::{Duration, Instant};
use std::ops::{Add, AddAssign, Sub, SubAssign};

// --- INFIX OPERATIONS ---

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(mut self, rhs: Duration) -> Instant {
        self += rhs;
        self
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(mut self, rhs: Duration) -> Instant {
        self -= rhs;
        self
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.since_start += rhs;
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.since_start -= rhs;
    }
}

// --- NON-CLOSED OPERATIONS ---

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Duration {
        self.since_start - rhs.since_start
    }
}
