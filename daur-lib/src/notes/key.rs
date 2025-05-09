use crate::notes::{Chroma, Sign};
use bitbag::{BitBag, Flags};
use std::fmt;
use std::fmt::{Display, Formatter};

/// A musical [key](https://en.wikipedia.org/wiki/Key_(music)).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Key {
    /// The tonic.
    pub tonic: Chroma,
    /// How the "black piano notes" should be displayed.
    pub sign: Sign,
    /// The intervals in the key (relative to the tonic).
    pub intervals: BitBag<KeyInterval>,
}

impl Default for Key {
    /// Returns _A minor_.
    ///
    /// A quite arbitrary choice.
    /// _A_, since it is the first letter of the latin alphabet.
    /// _Minor_, since it uses all the white keys on the piano.
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
