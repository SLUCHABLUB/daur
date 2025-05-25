use std::ops::{Add, AddAssign, Mul, MulAssign};

/// A duration of sample time. A sample count.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Duration {
    /// The number of samples that fit in the duration.
    pub samples: usize,
}

impl Duration {
    /// 0.
    pub const ZERO: Duration = Duration { samples: 0 };
}

// TODO: derive
impl Add for Duration {
    type Output = Duration;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        self.samples = self.samples.saturating_add(rhs.samples);
    }
}

// TODO: derive
impl Mul<usize> for Duration {
    type Output = Duration;

    fn mul(mut self, rhs: usize) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<usize> for Duration {
    fn mul_assign(&mut self, rhs: usize) {
        self.samples = self.samples.saturating_mul(rhs);
    }
}
