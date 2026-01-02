use crate::Ratio;
use crate::audio::sample;
use crate::time;
use std::ops::Div;

mod ops;

/// A duration of sample time. A sample count.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Duration {
    /// The number of samples that fit in the duration.
    pub samples: usize,
}

impl Duration {
    /// 0.
    pub const ZERO: Duration = Duration { samples: 0 };

    /// One sample.
    pub const SAMPLE: Duration = Duration { samples: 1 };
}

impl Div<sample::Rate> for Duration {
    type Output = time::Duration;

    fn div(self, rhs: sample::Rate) -> time::Duration {
        #![expect(
            clippy::suspicious_arithmetic_impl,
            reason = "`sample_duration` is the reciprocal"
        )]
        rhs.sample_duration().get() * Ratio::from_usize(self.samples)
    }
}
