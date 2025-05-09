use crate::notes::{Chroma, KeyInterval, Sign};
use bitbag::BitBag;
use core::fmt;
use core::fmt::{Display, Formatter};

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
