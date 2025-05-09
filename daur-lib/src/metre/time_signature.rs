use crate::NonZeroRatio;
use crate::metre::{Bar, Changing, Instant, NonZeroDuration};
use core::fmt;
use core::fmt::{Display, Formatter};
use core::iter::from_fn;
use core::num::{NonZeroU8, NonZeroU64};
use non_zero::non_zero;

/// A time signature.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TimeSignature {
    /// The upper number of the time signature.
    /// The number of beats that fit in a bar.
    pub beats_per_bar: NonZeroU8,
    /// The lower number of the time signature.
    /// The number of beats that fit in a whole note.
    pub beats_per_whole_note: NonZeroU8,
}

impl TimeSignature {
    /// The duration of a bar.
    #[must_use]
    pub fn bar_duration(self) -> NonZeroDuration {
        NonZeroDuration {
            whole_notes: NonZeroRatio::new(
                NonZeroU64::from(self.beats_per_bar),
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
            beats_per_bar: non_zero!(4),
            beats_per_whole_note: non_zero!(4),
        }
    }
}

impl Display for TimeSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.beats_per_bar, self.beats_per_whole_note)
    }
}

impl Changing<TimeSignature> {
    pub(crate) fn first_bar(&self) -> Bar {
        Bar {
            start: Instant::START,
            time_signature: self.start,
        }
    }

    pub(crate) fn bars(&self) -> impl Iterator<Item = Bar> + use<'_> {
        let mut start = Instant::START;

        from_fn(move || {
            let bar = Bar {
                start,
                time_signature: self.get(start),
            };

            start += bar.duration();

            Some(bar)
        })
    }

    pub(crate) fn bar_n(&self, index: usize) -> Bar {
        #[expect(clippy::unwrap_used, reason = "`bars()` never returns `None`")]
        self.bars().nth(index).unwrap()
    }
}
