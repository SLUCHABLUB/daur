use crate::musical_time::{
    Duration, Instant, NonZeroInstant, NonZeroPeriod, Period, Signature, Tempo,
};
use crate::{Changing, real_time};
use itertools::{chain, min};
use std::iter::from_fn;
use std::sync::Arc;

/// A mapping between real and musical time
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Mapping {
    /// The tempo of the music.
    pub tempo: Arc<Changing<Tempo>>,
    /// The time signature of the music.
    pub time_signature: Arc<Changing<Signature>>,
}

impl Mapping {
    /// Calculates a real-time instant from a musical instant.
    #[must_use]
    pub fn real_time(&self, instant: Instant) -> real_time::Instant {
        let mut since_start = real_time::Duration::ZERO;

        for period in self.time_constant_periods(Instant::START, Some(instant)) {
            let tempo = self.tempo.get(period.start);
            let time_signature = self.time_signature.get(period.start);

            let beat_count = period.duration.get() / time_signature.beat_duration();

            since_start += tempo.beat_duration().get() * beat_count;
        }

        real_time::Instant { since_start }
    }

    /// Calculates a musical instant from a real-time instant.
    #[must_use]
    pub fn musical(&self, instant: real_time::Instant) -> Instant {
        self.period(Instant::START, instant.since_start).end()
    }

    /// Calculates a period from a starting point and a real-time duration.
    #[must_use]
    pub fn period(&self, start: Instant, duration: real_time::Duration) -> Period {
        let mut remaining = duration;
        let mut duration = Duration::ZERO;

        let mut last = start;

        for period in self.time_constant_periods(start, None) {
            let tempo = self.tempo.get(period.start);
            let time_signature = self.time_signature.get(period.start);

            let beat_count = period.duration / time_signature.beat_duration();

            let full_duration = tempo.beat_duration() * beat_count;

            if remaining < full_duration.get() {
                let fraction = remaining / full_duration;
                duration += period.duration.get() * fraction;
                remaining = real_time::Duration::ZERO;
                break;
            }

            remaining -= full_duration.get();
            duration += period.duration.get();

            last = period.get().end();
        }

        // The period extends after all tempo and time-signature changes
        if remaining != real_time::Duration::ZERO {
            let tempo = self.tempo.get(last);
            let time_signature = self.time_signature.get(last);

            let beat_count = remaining / tempo.beat_duration();

            duration += time_signature.beat_duration().get() * beat_count;
        }

        Period { start, duration }
    }

    /// Returns the periods that have a constant tempo and time-signature.
    ///
    /// If an end point is specified, it will be the end point if the last period.
    fn time_constant_periods(
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
