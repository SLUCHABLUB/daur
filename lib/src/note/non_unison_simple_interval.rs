use enum_iterator::Sequence;
use enumset::EnumSet;
use enumset::EnumSetType;
use enumset::enum_set;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

// TODO: don't use short names?
/// Positive intervals less than an octave and greater than a perfect unison.
#[expect(
    non_camel_case_types,
    reason = "the (standardised?) short names for intervals require casing for distinction"
)]
#[derive(EnumSetType, Hash, Debug, Sequence, Serialize, Deserialize)]
#[enumset(serialize_repr = "list")]
pub enum NonUnisonSimpleInterval {
    /// A minor second.
    m2,
    /// A major second.
    M2,
    /// A minor third.
    m3,
    /// A major third.
    M3,
    /// A perfect fourth.
    P4,
    /// A tritone.
    TT,
    /// A perfect fifth.
    P5,
    /// A minor sixth.
    m6,
    /// A major eighth.
    M6,
    /// A minor seventh.
    m7,
    /// A major seventh.
    M7,
}

impl NonUnisonSimpleInterval {
    /// The intervals in the minor key.
    pub const MINOR: EnumSet<NonUnisonSimpleInterval> = enum_set!(
        NonUnisonSimpleInterval::M2
            | NonUnisonSimpleInterval::m3
            | NonUnisonSimpleInterval::P4
            | NonUnisonSimpleInterval::P5
            | NonUnisonSimpleInterval::m6
            | NonUnisonSimpleInterval::m7
    );

    /// Returns the name for a collection of intervals.
    #[must_use]
    pub fn collection_name(intervals: EnumSet<NonUnisonSimpleInterval>) -> &'static str {
        if intervals == NonUnisonSimpleInterval::MINOR {
            "minor"
        } else {
            "custom"
        }
    }
}

impl Display for NonUnisonSimpleInterval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #![expect(clippy::use_debug, reason = "Display = Debug here")]
        write!(f, "{self:?}")
    }
}
