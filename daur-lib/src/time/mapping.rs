use crate::Changing;
use crate::time::{
    Duration, Instant, NonZeroInstant, NonZeroPeriod, Period, Signature, Tempo, real,
};
use itertools::{chain, min};
use std::iter::from_fn;
use std::sync::Arc;

/// A mapping between real and musical time
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Mapping {
    /// The tempo of the music
    pub tempo: Arc<Changing<Tempo>>,
    /// The time signature of the music
    pub time_signature: Arc<Changing<Signature>>,
}

impl Mapping {
    /// Calculates the real time duration that has elapsed since [the start](Instant::START).
    #[must_use]
    pub fn real_time_offset(&self, instant: Instant) -> real::Duration {
        self.real_time_duration(Period {
            start: Instant::START,
            duration: instant.since_start,
        })
    }

    /// Calculates the real time duration of a period.
    #[must_use]
    pub fn real_time_duration(&self, period: Period) -> real::Duration {
        let mut duration = real::Duration::ZERO;

        for period in self.periods(period.start, Some(period.end())) {
            let tempo = self.tempo.get(period.start);
            let time_signature = self.time_signature.get(period.start);

            let beat_count = period.duration.get() / time_signature.beat_duration();

            duration += tempo.beat_duration().get() * beat_count;
        }

        duration
    }

    /// Calculates a period from a starting point and a real-time duration.
    #[must_use]
    pub fn period(&self, start: Instant, duration: real::Duration) -> Period {
        let mut remaining = duration;
        let mut duration = Duration::ZERO;

        let mut last = start;

        for period in self.periods(start, None) {
            let tempo = self.tempo.get(period.start);
            let time_signature = self.time_signature.get(period.start);

            let beat_count = period.duration / time_signature.beat_duration();

            let full_duration = tempo.beat_duration() * beat_count;

            if remaining < full_duration.get() {
                let fraction = remaining / full_duration;
                duration += period.duration.get() * fraction;
                remaining = real::Duration::ZERO;
                break;
            }

            remaining -= full_duration.get();
            duration += period.duration.get();

            last = period.get().end();
        }

        // The period extends after all tempo and time-signature changes
        if remaining != real::Duration::ZERO {
            let tempo = self.tempo.get(last);
            let time_signature = self.time_signature.get(last);

            let beat_count = remaining / tempo.beat_duration();

            duration += time_signature.beat_duration().get() * beat_count;
        }

        Period { start, duration }
    }

    fn periods(
        &self,
        mut start: Instant,
        end: Option<Instant>,
    ) -> impl Iterator<Item = NonZeroPeriod> + use<'_> {
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

            let period = NonZeroPeriod::from_endpoints(start, next);

            start = next;

            period
        })
    }
}
