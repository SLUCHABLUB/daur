use crate::project::changing::Changing;
use crate::ratio::Ratio;
use crate::time::bar::Bar;
use crate::time::duration::Duration;
use crate::time::instant::Instant;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::from_fn;
use std::num::NonZeroU8;

#[expect(clippy::unwrap_used, reason = "4 is not 0")]
const FOUR: NonZeroU8 = NonZeroU8::new(4).unwrap();

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TimeSignature {
    pub beats_per_bar: NonZeroU8,
    pub beat_size: NonZeroU8,
}

impl TimeSignature {
    pub fn bar_duration(self) -> Duration {
        Duration {
            whole_notes: Ratio::new(self.beats_per_bar.get().into(), self.beat_size.get().into()),
        }
    }

    pub fn beat_duration(self) -> Duration {
        self.bar_duration() / Ratio::from(self.beats_per_bar)
    }
}

/// "Common" time
impl Default for TimeSignature {
    fn default() -> Self {
        TimeSignature {
            beats_per_bar: FOUR,
            beat_size: FOUR,
        }
    }
}

impl Display for TimeSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.beats_per_bar, self.beat_size)
    }
}

impl Changing<TimeSignature> {
    pub fn bars(&self) -> impl Iterator<Item = Bar> + use<'_> {
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
}
