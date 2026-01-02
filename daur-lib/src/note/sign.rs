use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write as _;
use std::ops::Not;
use strum::VariantArray;

pub(super) const SHARP: char = '\u{266F}';
pub(super) const FLAT: char = '\u{266D}';

/// A flat or sharp sign.
#[derive(
    Copy, Clone, Eq, PartialEq, Hash, Debug, Default, VariantArray, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Sign {
    /// ♯
    #[default]
    #[serde(alias = "#", alias = "\u{266f}")]
    Sharp,
    /// ♭
    #[serde(alias = "b", alias = "\u{266d}")]
    Flat,
}

impl Display for Sign {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Sign::Sharp => f.write_char(SHARP),
            Sign::Flat => f.write_char(FLAT),
        }
    }
}

impl Not for Sign {
    type Output = Sign;

    fn not(self) -> Sign {
        match self {
            Sign::Sharp => Sign::Flat,
            Sign::Flat => Sign::Sharp,
        }
    }
}
