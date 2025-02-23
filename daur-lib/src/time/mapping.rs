use crate::project::changing::Changing;
use crate::time::{Duration, Instant, NonZeroInstant, Period, Signature, Tempo};
use crate::Ratio;
use itertools::{chain, min};
use std::iter::from_fn;
use std::sync::Arc;
use std::time;

/// A mapping between real and musical time
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Mapping {
    /// The tempo of the music
    pub tempo: Arc<Changing<Tempo>>,
    /// The time signature of the music
    pub time_signature: Arc<Changing<Signature>>,
}

impl Mapping {
    /// The real time duration between [`Instant::START`] and `instant`
    #[must_use]
    pub fn real_time_offset(&self, instant: Instant) -> time::Duration {
        self.real_time_duration(Period {
            start: Instant::START,
            duration: instant.since_start,
        })
    }

    /// The real time duration of `period`
    #[must_use]
    pub fn real_time_duration(&self, period: Period) -> time::Duration {
        let mut duration = time::Duration::ZERO;

        for period in self.periods(period.start, Some(period.end())) {
            let tempo = self.tempo.get(period.start);
            let time_signature = self.time_signature.get(period.start);

            let beat_count = period.duration / time_signature.beat_duration();

            let seconds = tempo.beat_duration().as_secs_f64() * beat_count.to_float();

            duration = duration.saturating_add(time::Duration::from_secs_f64(seconds));
        }

        duration
    }

    /// A period starting at `start` with a duration of `duration`
    #[must_use]
    pub fn period(&self, start: Instant, duration: time::Duration) -> Period {
        let mut remaining = duration;
        let mut duration = Duration::ZERO;

        let mut last = start;

        for period in self.periods(start, None) {
            let tempo = self.tempo.get(period.start);
            let time_signature = self.time_signature.get(period.start);

            let beat_count = period.duration / time_signature.beat_duration();

            let seconds = tempo.beat_duration().as_secs_f64() * beat_count.to_float();

            let full_duration = time::Duration::from_secs_f64(seconds);

            if remaining < full_duration {
                let fraction = remaining.as_secs_f64() / seconds;
                let fraction = Ratio::approximate(fraction);

                remaining = time::Duration::ZERO;
                duration += period.duration * fraction;
                break;
            }

            remaining = remaining.saturating_sub(full_duration);
            duration += period.duration;

            last = period.end();
        }

        // The period extends after all tempo and time-signature changes
        if remaining != time::Duration::ZERO {
            let tempo = self.tempo.get(last);
            let time_signature = self.time_signature.get(last);

            let beat_count = remaining.as_secs_f64() / tempo.beat_duration().as_secs_f64();
            let beat_count = Ratio::approximate(beat_count);

            duration += time_signature.beat_duration().get() * beat_count;
        }

        Period { start, duration }
    }

    fn periods(
        &self,
        mut start: Instant,
        end: Option<Instant>,
    ) -> impl Iterator<Item = Period> + use<'_> {
        from_fn(move || {
            if let Some(end) = end {
                if end < start {
                    return None;
                }
            }

            let next_tempo = self
                .tempo
                .changes
                .keys()
                .copied()
                .map(NonZeroInstant::get)
                .find(|instant| start < *instant);
            let next_time_signature = self
                .time_signature
                .changes
                .keys()
                .copied()
                .map(NonZeroInstant::get)
                .find(|instant| start < *instant);

            let next = min(chain!(end, next_tempo, next_time_signature))?;

            let period = Period {
                start,
                duration: next - start,
            };

            start = next;

            Some(period)
        })
    }
}
