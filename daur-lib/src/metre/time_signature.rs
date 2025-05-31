use crate::NonZeroRatio;
use crate::metre::{Changing, Instant, Measure, NonZeroDuration};
use non_zero::non_zero;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::from_fn;
use std::num::{NonZeroU8, NonZeroU64};

/// A time signature.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TimeSignature {
    /// The upper number of the time signature.
    /// The number of beats that fit in a measure.
    pub beats_per_measure: NonZeroU8,
    /// The lower number of the time signature.
    /// The number of beats that fit in a whole note.
    pub beats_per_whole_note: NonZeroU8,
}

impl TimeSignature {
    /// The duration of a measure.
    #[must_use]
    pub fn measure_duration(self) -> NonZeroDuration {
        NonZeroDuration {
            whole_notes: NonZeroRatio::new(
                NonZeroU64::from(self.beats_per_measure),
                NonZeroU64::from(self.beats_per_whole_note),
            ),
        }
    }

    /// The duration of a beat.
    #[must_use]
    pub fn beat_duration(self) -> NonZeroDuration {
        NonZeroDuration {
            whole_notes: NonZeroRatio::reciprocal_of(NonZeroU64::from(self.beats_per_whole_note)),
        }
    }
}

impl Default for TimeSignature {
    /// Returns _common time_ (4/4).
    fn default() -> Self {
        TimeSignature {
            beats_per_measure: non_zero!(4),
            beats_per_whole_note: non_zero!(4),
        }
    }
}

impl Display for TimeSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}",
            self.beats_per_measure, self.beats_per_whole_note
        )
    }
}

impl Changing<TimeSignature> {
    pub(crate) fn first_measure(&self) -> Measure {
        Measure {
            start: Instant::START,
            time_signature: self.start,
        }
    }

    pub(crate) fn measures(&self) -> impl Iterator<Item = Measure> + use<'_> {
        let mut start = Instant::START;

        from_fn(move || {
            let measure = Measure {
                start,
                time_signature: self.get(start),
            };

            start += measure.duration();

            Some(measure)
        })
    }

    pub(crate) fn measure_n(&self, index: usize) -> Measure {
        #[expect(clippy::unwrap_used, reason = "`bars()` never returns `None`")]
        self.measures().nth(index).unwrap()
    }
}
