use crate::metre::{Changing, Duration, Instant, Period, Quantisation, TimeSignature};
use crate::ui::Length;

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
    pub fn duration(self) -> Duration {
        self.time_signature.measure_duration().get()
    }

    /// Returns the period of the measure.
    #[must_use]
    pub fn period(self) -> Period {
        Period {
            start: self.start,
            duration: self.duration(),
        }
    }

    pub(crate) fn next(self, time_signature: &Changing<TimeSignature>) -> Measure {
        let start = self.period().end();
        Measure {
            start,
            time_signature: time_signature.get(start),
        }
    }

    pub(crate) fn width(&self, quantisation: Quantisation) -> Length {
        quantisation.cell_width.get() * (self.duration() / quantisation.cell_duration).ceiled()
    }
}
