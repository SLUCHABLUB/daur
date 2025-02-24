use crate::project::changing::Changing;
use crate::ratio::NonZeroRatio;
use crate::time::bar::Bar;
use crate::time::duration::NonZeroDuration;
use crate::time::instant::Instant;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::from_fn;
use std::num::{NonZeroU32, NonZeroU8};

#[expect(clippy::unwrap_used, reason = "4 is not 0")]
const FOUR: NonZeroU8 = NonZeroU8::new(4).unwrap();

/// A time signature
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    /// The upper number of the time signature.
    /// How many beats fit in a bar.
    pub beats_per_bar: NonZeroU8,
    /// The lower number of the time signature.
    /// How many beats fit in a whole note.
    pub beat_size: NonZeroU8,
}

impl Signature {
    /// The duration of one bar
    #[must_use]
    pub fn bar_duration(self) -> NonZeroDuration {
        NonZeroDuration {
            whole_notes: NonZeroRatio::new(
                NonZeroU32::from(self.beats_per_bar),
                NonZeroU32::from(self.beat_size),
            ),
        }
    }

    /// The duration of one beat
    #[must_use]
    pub fn beat_duration(self) -> NonZeroDuration {
        self.bar_duration() / NonZeroRatio::integer(NonZeroU32::from(self.beats_per_bar))
    }
}

/// "Common" time
impl Default for Signature {
    fn default() -> Self {
        Signature {
            beats_per_bar: FOUR,
            beat_size: FOUR,
        }
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.beats_per_bar, self.beat_size)
    }
}

impl Changing<Signature> {
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

    pub fn bar_n(&self, index: usize) -> Bar {
        #[expect(clippy::unwrap_used, reason = "`bars()` never returns `None`")]
        self.bars().nth(index).unwrap()
    }
}
