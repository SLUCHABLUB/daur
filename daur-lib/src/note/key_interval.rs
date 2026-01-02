use enumset::EnumSet;
use enumset::EnumSetType;
use enumset::enum_set;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use strum::VariantArray;

// TODO: rename to `NonUnisonSimpleInterval`
// TODO: don't use short names?
/// Positive intervals less than an octave and greater than a perfect unison.
#[expect(
    non_camel_case_types,
    reason = "the (standardised?) short names for intervals require casing for distinction"
)]
#[derive(EnumSetType, Hash, Debug, VariantArray, Serialize, Deserialize)]
#[enumset(serialize_repr = "list")]
pub enum KeyInterval {
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

impl KeyInterval {
    /// The intervals in the minor key.
    pub const MINOR: EnumSet<KeyInterval> = enum_set!(
        KeyInterval::M2
            | KeyInterval::m3
            | KeyInterval::P4
            | KeyInterval::P5
            | KeyInterval::m6
            | KeyInterval::m7
    );

    /// Returns the name for a collection of intervals.
    #[must_use]
    pub fn name(intervals: EnumSet<KeyInterval>) -> &'static str {
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
