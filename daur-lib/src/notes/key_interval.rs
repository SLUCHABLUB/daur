use bitbag::{BitBag, Flags};
use core::fmt;
use core::fmt::{Display, Formatter};

/// Intervals less than an octave and greater than a perfect unison.
#[expect(
    non_camel_case_types,
    reason = "the (standardised?) short names for intervals require casing for distinction"
)]
#[derive(Flags)]
#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyInterval {
    /// A minor second.
    m2 = 1 << 0,
    /// A major second.
    M2 = 1 << 1,
    /// A minor third.
    m3 = 1 << 2,
    /// A major third.
    M3 = 1 << 3,
    /// A perfect fourth.
    P4 = 1 << 4,
    /// A tritone.
    TT = 1 << 5,
    /// A perfect fifth.
    P5 = 1 << 6,
    /// A minor sixth.
    m6 = 1 << 7,
    /// A major eighth.
    M6 = 1 << 8,
    /// A minor seventh.
    m7 = 1 << 9,
    /// A major seventh.
    M7 = 1 << 10,
}

impl KeyInterval {
    /// The intervals in the minor key.
    pub const MINOR: BitBag<KeyInterval> = BitBag::new_unchecked(
        KeyInterval::M2 as u16
            | KeyInterval::m3 as u16
            | KeyInterval::P4 as u16
            | KeyInterval::P5 as u16
            | KeyInterval::m6 as u16
            | KeyInterval::m7 as u16,
    );

    /// Returns the name for a collection of intervals.
    #[must_use]
    pub fn name(intervals: BitBag<KeyInterval>) -> &'static str {
        if intervals == KeyInterval::MINOR {
            "minor"
        } else {
            "custom"
        }
    }
}

impl Display for KeyInterval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #![expect(clippy::use_debug, reason = "Display = Debug here")]
        write!(f, "{self:?}")
    }
}
