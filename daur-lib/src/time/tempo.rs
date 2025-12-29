use crate::time::Duration;
use crate::time::NonZeroDuration;
use non_zero::non_zero;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::NonZeroU16;
use std::num::NonZeroU64;

/// A musical tempo.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Tempo {
    // TODO: support the psychopathy that is non-integral BPMs
    bpm: NonZeroU16,
}

impl Tempo {
    /// The duration of a beat at this tempo.
    #[must_use]
    pub fn beat_duration(self) -> NonZeroDuration {
        let bpm = NonZeroU64::from(self.bpm);

        // TODO: round.
        #[expect(clippy::integer_division, reason = "see TODO")]
        let nanoseconds = Duration::MINUTE.nanoseconds / bpm;

        // The minimum value of `nanoseconds` is 15,259, so we could technically unwrap.
        let nanoseconds = NonZeroU64::new(nanoseconds).unwrap_or(NonZeroU64::MIN);

        NonZeroDuration { nanoseconds }
    }
}

impl Default for Tempo {
    /// Returns 180 beats per minute.
    ///
    /// A quite arbitrary choice.
    // TODO: rationale
    fn default() -> Tempo {
        Tempo {
            bpm: non_zero!(180),
        }
    }
}

impl Display for Tempo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.bpm)
    }
}
