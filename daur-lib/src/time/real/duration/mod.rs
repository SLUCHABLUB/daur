mod non_zero;

use crate::Ratio;
pub use non_zero::NonZeroDuration;
use std::num::NonZeroU128;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::time;

/// A duration of real time.
///
/// Like [`core::time::Duration`], it can only represent time down to nanoseconds.
/// Furthermore, the maximum duration which is representable is only about 500 years.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Duration {
    /// The number of nanoseconds that the duration takes up.
    pub nanoseconds: u64,
}

impl Duration {
    /// 0
    pub const ZERO: Duration = Duration { nanoseconds: 0 };

    /// One second.
    pub const SECOND: Duration = Duration {
        nanoseconds: 1_000_000_000,
    };

    /// One minute.
    pub const MINUTE: Duration = Duration {
        nanoseconds: 60_000_000_000,
    };
}

impl From<time::Duration> for Duration {
    fn from(duration: time::Duration) -> Self {
        let nanoseconds = u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX);

        Duration { nanoseconds }
    }
}

impl From<Duration> for time::Duration {
    fn from(duration: Duration) -> Self {
        time::Duration::from_nanos(duration.nanoseconds)
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Self) -> Duration {
        Duration {
            nanoseconds: self.nanoseconds.saturating_add(rhs.nanoseconds),
        }
    }
}

// TODO: derive
impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Duration {
        Duration {
            nanoseconds: self.nanoseconds.saturating_sub(rhs.nanoseconds),
        }
    }
}

// TODO: derive
impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<Ratio> for Duration {
    type Output = Duration;

    fn mul(self, rhs: Ratio) -> Duration {
        let nanoseconds = u128::from(self.nanoseconds);
        let numerator = u128::from(rhs.numerator());
        let denominator = NonZeroU128::from(rhs.denominator());

        // TODO: round
        #[expect(clippy::arithmetic_side_effects, reason = "we encapsulate in u128")]
        let nanoseconds = nanoseconds * numerator / denominator;
        let nanoseconds = u64::try_from(nanoseconds).unwrap_or(u64::MAX);

        Duration { nanoseconds }
    }
}

impl Div<NonZeroDuration> for Duration {
    type Output = Ratio;

    fn div(self, rhs: NonZeroDuration) -> Ratio {
        Ratio::approximate_big(
            u128::from(self.nanoseconds),
            NonZeroU128::from(rhs.nanoseconds),
        )
    }
}
