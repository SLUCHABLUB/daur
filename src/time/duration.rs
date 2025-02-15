use crate::time::Ratio;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Duration {
    pub whole_notes: Ratio,
}

impl Duration {
    pub const ZERO: Duration = Duration {
        whole_notes: Ratio::ZERO,
    };

    pub const QUARTER: Duration = Duration {
        whole_notes: Ratio::QUARTER,
    };
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(mut self, rhs: Duration) -> Duration {
        self += rhs;
        self
    }
}

impl AddAssign<Duration> for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        self.whole_notes += rhs.whole_notes;
    }
}

impl Mul<Ratio> for Duration {
    type Output = Duration;

    fn mul(mut self, rhs: Ratio) -> Duration {
        self *= rhs;
        self
    }
}

impl MulAssign<Ratio> for Duration {
    fn mul_assign(&mut self, rhs: Ratio) {
        self.whole_notes *= rhs;
    }
}

impl Div<Duration> for Duration {
    type Output = Ratio;

    fn div(self, rhs: Duration) -> Ratio {
        self.whole_notes / rhs.whole_notes
    }
}

impl Div<Ratio> for Duration {
    type Output = Duration;

    fn div(self, rhs: Ratio) -> Duration {
        Duration {
            whole_notes: self.whole_notes / rhs,
        }
    }
}
