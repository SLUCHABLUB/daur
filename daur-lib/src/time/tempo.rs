use crate::time::{Duration, NonZeroDuration};
use core::fmt;
use core::fmt::{Display, Formatter};
use core::num::{NonZeroU16, NonZeroU64};

// TODO move to real time
// TODO: make transparent
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
    fn default() -> Self {
        Tempo {
            #[expect(clippy::unwrap_used, reason = "180 != 0")]
            bpm: NonZeroU16::new(180).unwrap(),
        }
    }
}

impl Display for Tempo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.bpm)
    }
}
