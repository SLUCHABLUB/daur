use crate::NonZeroRatio;
use crate::time::real::Duration;
use std::num::{NonZeroU64, NonZeroU128};
use std::ops::Mul;

/// A non-zero [duration of real time](Duration).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonZeroDuration {
    /// The number of nanoseconds that the duration takes up.
    pub nanoseconds: NonZeroU64,
}

impl NonZeroDuration {
    /// One second.
    #[expect(clippy::unwrap_used, reason = "1 second != 0")]
    pub const SECOND: NonZeroDuration = NonZeroDuration::from_duration(Duration::SECOND).unwrap();

    /// Converts the duration to a [zeroable one](Duration).
    #[must_use]
    pub fn get(self) -> Duration {
        Duration {
            nanoseconds: self.nanoseconds.get(),
        }
    }

    /// Converts a duration to a non-zero one if it is not zero.
    #[must_use]
    pub const fn from_duration(duration: Duration) -> Option<NonZeroDuration> {
        match NonZeroU64::new(duration.nanoseconds) {
            Some(nanoseconds) => Some(NonZeroDuration { nanoseconds }),
            None => None,
        }
    }
}

impl Mul<NonZeroRatio> for NonZeroDuration {
    type Output = NonZeroDuration;

    fn mul(self, rhs: NonZeroRatio) -> NonZeroDuration {
        let nanoseconds = NonZeroU128::from(self.nanoseconds);
        let numerator = NonZeroU128::from(rhs.numerator());
        let denominator = NonZeroU128::from(rhs.denominator());

        // TODO: round
        #[expect(clippy::suspicious_arithmetic_impl, reason = "we multiply by a ratio")]
        let nanoseconds = nanoseconds.saturating_mul(numerator).get() / denominator;
        let nanoseconds = u64::try_from(nanoseconds).unwrap_or(u64::MAX);
        let nanoseconds = NonZeroU64::new(nanoseconds).unwrap_or(NonZeroU64::MIN);

        NonZeroDuration { nanoseconds }
    }
}
