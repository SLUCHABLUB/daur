use crate::project::changing::Changing;
use crate::ratio::Ratio;
use crate::time::duration::Duration;
use crate::time::instant::Instant;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use std::ops::Range;
use std::time;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Period {
    pub start: Instant,
    pub duration: Duration,
}

impl Period {
    pub fn from_real_time(
        start: Instant,
        time_signature: &Changing<TimeSignature>,
        tempo: &Changing<Tempo>,
        duration: time::Duration,
    ) -> Period {
        let mut remaining = duration;
        let mut duration = Duration::ZERO;

        let mut current = start;

        for bar in time_signature.bars() {
            if bar.period().end() < current {
                continue;
            }
            if remaining == time::Duration::ZERO {
                break;
            }

            // The (remaining) duration of the bar.
            // (A period with constant time signature)
            let period = Period {
                start: current,
                duration: Duration::min(bar.duration(), bar.period().end() - current),
            };

            for (period, tempo) in tempo.tempo_constant_periods(period) {
                let real_time_duration =
                    tempo.period_to_real_time_duration(period, bar.time_signature);

                #[expect(clippy::arithmetic_side_effects, reason = "checked in if statement")]
                if real_time_duration <= remaining {
                    duration += period.duration;
                    remaining -= real_time_duration;
                } else {
                    let fraction = remaining.as_secs_f64() / real_time_duration.as_secs_f64();

                    duration += period.duration * Ratio::approximate(fraction);
                    remaining = time::Duration::ZERO;
                    break;
                }
            }

            current += period.duration;
        }

        Period { start, duration }
    }

    pub fn real_time_duration(
        self,
        time_signature: &Changing<TimeSignature>,
        tempo: &Changing<Tempo>,
    ) -> time::Duration {
        let mut duration = time::Duration::ZERO;

        let mut current = self.start;

        for bar in time_signature.bars() {
            if bar.period().end() < current {
                continue;
            }

            let end = Instant::min(bar.period().end(), self.end());

            // The (remaining) duration of the bar.
            // (A period with constant time signature)
            let period = Period {
                start: current,
                duration: Duration::min(bar.duration(), end - current),
            };

            for (period, tempo) in tempo.tempo_constant_periods(period) {
                let period_duration =
                    tempo.period_to_real_time_duration(period, bar.time_signature);
                duration = duration.saturating_add(period_duration);
            }

            if self.end() <= bar.period().end() {
                break;
            }

            current += period.duration;
        }

        duration
    }

    fn range(self) -> Range<Instant> {
        Range {
            start: self.start,
            end: self.end(),
        }
    }

    pub fn end(&self) -> Instant {
        self.start + self.duration
    }

    pub fn contains(self, instant: Instant) -> bool {
        self.range().contains(&instant)
    }
}
