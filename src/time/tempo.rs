use crate::project::changing::Changing;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;
use ordered_float::NotNan;
use std::ops::Bound;
use std::time::Duration;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Tempo {
    // INVARIANT: is positive
    bpm: NotNan<f64>,
}

impl Tempo {
    fn beat_in_seconds(self) -> f64 {
        self.bpm.recip() * 60.0
    }

    pub fn period_to_real_time_duration(
        self,
        period: Period,
        time_signature: TimeSignature,
    ) -> Duration {
        let beat_count = period.duration / time_signature.beat_duration();
        let seconds = beat_count.to_float() * self.beat_in_seconds();
        Duration::from_secs_f64(seconds)
    }
}

// TODO: rationale
impl Default for Tempo {
    fn default() -> Self {
        Tempo {
            bpm: NotNan::new(180.0).unwrap(),
        }
    }
}

impl Changing<Tempo> {
    /// Subdivides the period into periods with constant tempo
    pub fn tempo_constant_periods(&self, period: Period) -> Vec<(Period, Tempo)> {
        let tempo = self[period.start];
        let mut periods = vec![(period, tempo)];

        for (instant, tempo) in self
            .changes
            .range((Bound::Excluded(period.start), Bound::Unbounded))
        {
            if period.end() <= *instant {
                break;
            }

            let (last, _) = periods.last_mut().unwrap();

            last.duration = *instant - last.start;

            let duration = period.end() - *instant;

            periods.push((
                Period {
                    start: *instant,
                    duration,
                },
                *tempo,
            ));
        }

        periods
    }
}
