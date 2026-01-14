//! Items pertaining to [`Measure`].

use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::NonZeroDuration;
use crate::metre::NonZeroPeriod;
use crate::metre::Quantisation;
use crate::metre::TimeSignature;
use crate::ui::Length;
use std::num::NonZeroU64;

/// A [measure](https://en.wikipedia.org/wiki/Measure_(music)).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Measure {
    /// The start of the measure.
    pub start: Instant,
    /// The time signature of the measure.
    pub time_signature: TimeSignature,
}

impl Measure {
    /// Returns the duration of the measure.
    #[must_use]
    pub fn duration(self) -> NonZeroDuration {
        self.time_signature.measure_duration()
    }

    /// Returns the period of the measure.
    #[must_use]
    pub fn period(self) -> NonZeroPeriod {
        NonZeroPeriod {
            start: self.start,
            duration: self.duration(),
        }
    }

    /// Returns the next measure.
    pub(crate) fn next(self, time_signature: &Changing<TimeSignature>) -> Measure {
        let start = self.period().get().end();
        Measure {
            start,
            time_signature: time_signature.get(start),
        }
    }

    /// The number of cells to divide the measure into.
    pub(crate) fn cell_count(self, quantisation: Quantisation) -> NonZeroU64 {
        (self.duration() / quantisation.cell_duration).ceiling()
    }

    /// The width of the measure.
    pub(crate) fn width(self, quantisation: Quantisation) -> Length {
        quantisation.cell_width.get() * (self.duration() / quantisation.cell_duration).ceiling()
    }
}
