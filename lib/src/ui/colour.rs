//! Items pertaining to [`Colour`].

use serde::Deserialize;
use serde::Serialize;

// TODO: Use `typed_colours` for representation and serde.
/// A colour.
/// Operations on a [`Colour`] use the Oklab colour space unless specified otherwise.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Colour {
    /// The red channel.
    red: u8,
    /// The green channel.
    green: u8,
    /// The blue channel.
    blue: u8,
}

impl Colour {
    /// Converts the colour to sRGB.
    #[must_use]
    pub fn to_srgb(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }

    /// Pure black.
    pub(crate) const BLACK: Colour = Colour {
        red: 0,
        green: 0,
        blue: 0,
    };

    /// Pure white.
    pub(crate) const WHITE: Colour = Colour {
        red: 0xff,
        green: 0xff,
        blue: 0xff,
    };

    /// The web-colour "silver".
    pub(crate) const SILVER: Colour = Colour {
        red: 192,
        green: 192,
        blue: 192,
    };

    /// The web-colour "lime".
    pub(crate) const LIME: Colour = Colour {
        red: 0,
        green: 255,
        blue: 0,
    };

    /// The web-colour "magenta".
    pub(crate) const MAGENTA: Colour = Colour {
        red: 255,
        green: 0,
        blue: 255,
    };
}
