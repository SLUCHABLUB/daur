use crate::project::changing::Changing;
use crate::time::duration::Duration;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use crate::time::Ratio;
use num::Integer as _;
use saturating_cast::SaturatingCast as _;
use std::ops::{Add, AddAssign, Sub};
use std::time;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    pub whole_notes: Ratio,
}

impl Instant {
    pub const START: Instant = Instant {
        whole_notes: Ratio::ZERO,
    };

    fn real_time_duration_since_start(
        self,
        time_signature: &Changing<TimeSignature>,
        tempo: &Changing<Tempo>,
    ) -> time::Duration {
        let period = Period {
            start: Instant::START,
            duration: Duration {
                whole_notes: self.whole_notes,
            },
        };
        period.real_time_duration(time_signature, tempo)
    }

    pub fn to_sample(
        self,
        time_signature: &Changing<TimeSignature>,
        tempo: &Changing<Tempo>,
        sample_rate: u32,
    ) -> usize {
        const NANOS_PER_SECOND: u128 = 1_000_000_000;
        const HALF: u128 = 500_000_000;

        let sample_rate = u128::from(sample_rate);
        let duration = self.real_time_duration_since_start(time_signature, tempo);

        // < 2^64 * 10^9 < 2^94
        let nanos = duration.as_nanos();

        // * 2 since we are always in stereo
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "nanos < 2^94, sample_rate < 2^32 => nano_sample < 2^(94 + 32 + 1) < 2^128"
        )]
        let nano_sample = nanos * sample_rate * 2;

        let (mut sample, remainder) = nano_sample.div_rem(&NANOS_PER_SECOND);

        if remainder > HALF {
            sample = sample.saturating_add(1);
        }

        sample.saturating_cast()
    }
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
