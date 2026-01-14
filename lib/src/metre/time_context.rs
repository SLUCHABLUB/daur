//! Items pertaining to [`TimeContext`].

use crate::metre;
use crate::metre::Changing;
use crate::metre::TimeSignature;
use crate::time;
use crate::time::Tempo;
use std::cmp::min;
use std::collections::BTreeMap;
use std::ops::Div;
use std::ops::Mul;

/// Relevant information for converting between (metre)[metre] and [real time](time).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct TimeContext {
    /// The tempo.
    tempo: Tempo,
    /// The time signature.
    time_signature: TimeSignature,
}

impl Div<Tempo> for TimeSignature {
    type Output = TimeContext;

    fn div(self, rhs: Tempo) -> TimeContext {
        TimeContext {
            tempo: rhs,
            time_signature: self,
        }
    }
}

impl Div<&Changing<Tempo>> for &Changing<TimeSignature> {
    type Output = Changing<TimeContext>;

    fn div(self, rhs: &Changing<Tempo>) -> Changing<TimeContext> {
        let time_signature = self;
        let tempo = rhs;

        let mut tempo_iter = tempo.changes.iter();
        let mut time_signature_iter = time_signature.changes.iter();

        let mut current_tempo = tempo.start;
        let mut current_time_signature = time_signature.start;

        let mut next_tempo = tempo_iter.next();
        let mut next_time_signature = time_signature_iter.next();

        let mut output = Changing {
            start: current_time_signature / current_tempo,
            changes: BTreeMap::new(),
        };

        loop {
            match (next_tempo, next_time_signature) {
                (Some((tempo_change, tempo)), Some((time_signature_change, time_signature))) => {
                    let change = min(tempo_change, time_signature_change);

                    if change == tempo_change {
                        next_tempo = tempo_iter.next();
                        current_tempo = *tempo;
                    }
                    if change == time_signature_change {
                        next_time_signature = time_signature_iter.next();
                        current_time_signature = *time_signature;
                    }

                    output
                        .changes
                        .insert(*change, current_time_signature / current_tempo);
                }
                (Some((change, tempo)), None) => {
                    next_tempo = tempo_iter.next();
                    current_tempo = *tempo;

                    output
                        .changes
                        .insert(*change, current_time_signature / current_tempo);
                }
                (None, Some((change, time_signature))) => {
                    next_time_signature = time_signature_iter.next();
                    current_time_signature = *time_signature;

                    output
                        .changes
                        .insert(*change, current_time_signature / current_tempo);
                }
                (None, None) => break,
            }
        }

        output
    }
}

impl Mul<TimeContext> for metre::Duration {
    type Output = time::Duration;

    fn mul(self, rhs: TimeContext) -> time::Duration {
        rhs.tempo.beat_duration().get() * (self / rhs.time_signature.beat_duration())
    }
}

impl Mul<&Changing<TimeContext>> for metre::Instant {
    type Output = time::Instant;

    fn mul(self, rhs: &Changing<TimeContext>) -> time::Instant {
        let mut instant = time::Instant::START;

        let mut change = metre::Instant::START;
        let mut context = rhs.start;

        for (next_change, next_context) in &rhs.changes {
            if self < next_change.get() {
                break;
            }

            let duration = next_change.get() - change;
            instant += context.tempo.beat_duration().get()
                * (duration / context.time_signature.beat_duration());

            change = next_change.get();
            context = *next_context;
        }

        let remaining = self - change;
        instant += context.tempo.beat_duration().get()
            * (remaining / context.time_signature.beat_duration());

        instant
    }
}

impl Div<&Changing<TimeContext>> for time::Instant {
    type Output = metre::Instant;

    fn div(self, rhs: &Changing<TimeContext>) -> metre::Instant {
        let mut remaining = self.since_start;
        let mut instant = metre::Instant::START;

        let mut change = metre::Instant::START;
        let mut context = rhs.start;

        for (next_change, next_context) in &rhs.changes {
            let duration = next_change.get() - change;
            let real_duration = context.tempo.beat_duration().get()
                * (duration / context.time_signature.beat_duration());

            if remaining < real_duration {
                break;
            }

            instant += duration;
            remaining -= real_duration;

            change = next_change.get();
            context = *next_context;
        }

        instant += context.time_signature.beat_duration().get()
            * (remaining / context.tempo.beat_duration());

        instant
    }
}
