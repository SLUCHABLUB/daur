//! Items pertaining to [`OffsetMapping`].

use crate::Ratio;
use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::Quantisation;
use crate::metre::TimeSignature;
use crate::ui::Length;
use getset::CopyGetters;
use getset::Getters;
use std::convert::identity;

/// A mapping from [musical time](crate::metre) to a screen offset.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Getters, CopyGetters)]
pub struct OffsetMapping {
    // TODO: remove getter
    /// The time signature.
    #[get = "pub(crate)"]
    time_signature: Changing<TimeSignature>,
    // TODO: remove getter
    /// The quantisation.
    #[get_copy = "pub(crate)"]
    quantisation: Quantisation,
}

impl OffsetMapping {
    /// Constructs a new mapping.
    pub(crate) fn new(
        time_signature: Changing<TimeSignature>,
        quantisation: Quantisation,
    ) -> OffsetMapping {
        OffsetMapping {
            time_signature,
            quantisation,
        }
    }

    /// Maps an offset to an instant.
    #[must_use]
    pub fn instant(&self, offset: Length) -> Instant {
        self.instant_with_cell_count_rounding(offset, identity)
    }

    /// Like [`OffsetMapping::instant`] but the instant is quantised.
    #[must_use]
    pub fn quantised_instant(&self, offset: Length) -> Instant {
        self.instant_with_cell_count_rounding(offset, Ratio::rounded_half_down)
    }

    /// Maps an offset to an instant using a provided function for rounding.
    fn instant_with_cell_count_rounding(
        &self,
        offset: Length,
        round_cell_count: fn(Ratio) -> Ratio,
    ) -> Instant {
        let mut offset = offset;
        let mut instant = Instant::START;

        let mut measure = self.time_signature.first_measure();

        loop {
            let measure_width = measure.width(self.quantisation);

            if offset < measure_width {
                let cell_count = offset / self.quantisation.cell_width;
                instant += self.quantisation.cell_duration.get() * round_cell_count(cell_count);

                break;
            }

            offset -= measure_width;
            instant += measure.duration();

            measure = measure.next(&self.time_signature);
        }

        instant
    }

    /// Maps an instant to an offset.
    #[must_use]
    pub fn offset(&self, instant: Instant) -> Length {
        let mut remaining = instant.since_start;
        let mut offset = Length::ZERO;

        let mut measure = self.time_signature.first_measure();

        loop {
            if remaining < measure.duration().get() {
                let cell_count = remaining / self.quantisation.cell_duration;
                offset += self.quantisation.cell_width.get() * cell_count;

                break;
            }

            remaining -= measure.duration();
            offset += measure.width(self.quantisation);

            measure = measure.next(&self.time_signature);
        }

        offset
    }
}
