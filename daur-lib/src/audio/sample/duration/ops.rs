use crate::audio::sample::Duration;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

// --- INFIX OPERATIONS ---

impl<D: Into<Duration>> Add<D> for Duration {
    type Output = Duration;

    fn add(mut self, rhs: D) -> Duration {
        self += rhs;
        self
    }
}

impl<D: Into<Duration>> Sub<D> for Duration {
    type Output = Duration;

    fn sub(mut self, rhs: D) -> Duration {
        self -= rhs;
        self
    }
}

impl Mul<usize> for Duration {
    type Output = Duration;

    fn mul(mut self, rhs: usize) -> Duration {
        self *= rhs;
        self
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl<D: Into<Duration>> AddAssign<D> for Duration {
    fn add_assign(&mut self, rhs: D) {
        self.samples = self.samples.saturating_add(rhs.into().samples);
    }
}

impl<D: Into<Duration>> SubAssign<D> for Duration {
    fn sub_assign(&mut self, rhs: D) {
        self.samples = self.samples.saturating_sub(rhs.into().samples);
    }
}

impl MulAssign<usize> for Duration {
    fn mul_assign(&mut self, rhs: usize) {
        self.samples = self.samples.saturating_mul(rhs);
    }
}
