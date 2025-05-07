use crate::chroma::Chroma;
use crate::sign::Sign;
use bitbag::{BitBag, Flags};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Key {
    pub tonic: Chroma,
    /// How the "black piano notes" should be displayed
    pub sign: Sign,
    pub intervals: BitBag<KeyInterval>,
}

/// A quite arbitrary choice.
/// A, since it is the first letter in the latin alphabet.
/// Minor since it uses all the white keys on a piano.
impl Default for Key {
    fn default() -> Self {
        Key {
            tonic: Chroma::A,
            sign: Sign::default(),
            intervals: KeyInterval::MINOR,
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.tonic.name(self.sign),
            KeyInterval::name(self.intervals)
        )
    }
}

/// Intervals less than an octave and greater than a perfect unison.
#[expect(
    non_camel_case_types,
    reason = "the (standardised?) short names for intervals require casing for distinction"
)]
#[derive(Flags)]
#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyInterval {
    m2 = 1 << 0,
    M2 = 1 << 1,
    m3 = 1 << 2,
    M3 = 1 << 3,
    P4 = 1 << 4,
    TT = 1 << 5,
    P5 = 1 << 6,
    m6 = 1 << 7,
    M6 = 1 << 8,
    m7 = 1 << 9,
    M7 = 1 << 10,
}

impl KeyInterval {
    pub const MINOR: BitBag<KeyInterval> = BitBag::new_unchecked(
        KeyInterval::M2 as u16
            | KeyInterval::m3 as u16
            | KeyInterval::P4 as u16
            | KeyInterval::P5 as u16
            | KeyInterval::m6 as u16
            | KeyInterval::m7 as u16,
    );

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
