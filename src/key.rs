use crate::chroma::Chroma;
use crate::sign::Sign;
use bitbag::{BitBag, Flags};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Key {
    pub tonic: Chroma,
    /// How the "black piano notes" should be displayed
    pub sign: Sign,
    pub intervals: BitBag<KeyInterval>,
}

/// A quite arbitrary choice.
/// A, since it's the first letter in the latin alphabet.
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.tonic.name(self.sign),
            KeyInterval::name(self.intervals)
        )
    }
}

#[derive(Flags)]
#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyInterval {
    Minor2 = 1 << 0,
    Major2 = 1 << 1,
    Minor3 = 1 << 2,
    Major3 = 1 << 3,
    Perfect4 = 1 << 4,
    Tritone = 1 << 5,
    Perfect5 = 1 << 6,
    Minor6 = 1 << 7,
    Major6 = 1 << 8,
    Minor7 = 1 << 9,
    Major7 = 1 << 10,
}

impl KeyInterval {
    pub const MINOR: BitBag<KeyInterval> = BitBag::new_unchecked(
        KeyInterval::Major2 as u16
            | KeyInterval::Minor3 as u16
            | KeyInterval::Perfect4 as u16
            | KeyInterval::Perfect5 as u16
            | KeyInterval::Minor6 as u16
            | KeyInterval::Minor7 as u16,
    );

    pub fn name(intervals: BitBag<KeyInterval>) -> &'static str {
        if intervals == KeyInterval::MINOR {
            "minor"
        } else {
            "custom"
        }
    }
}
