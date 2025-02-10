use crate::project::changing::Changing;
use crate::time::duration::Duration;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use crate::time::Ratio;
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
        #![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]

        let sample_rate = f64::from(sample_rate);
        let seconds = self
            .real_time_duration_since_start(time_signature, tempo)
            .as_secs_f64();

        // * 2 since we are always in stereo
        (seconds * sample_rate).round() as usize * 2
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
