/// A colour represented in 8-bit-per-channel sRGB.
/// Operations on a [`Colour`] use the okLab colour space unless specified otherwise.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Colour {
    /// The red channel.
    pub red: u8,
    /// The green channel.
    pub green: u8,
    /// The blue channel.
    pub blue: u8,
}

impl Colour {
    /// Pure black.
    pub const BLACK: Colour = Colour {
        red: 0,
        green: 0,
        blue: 0,
    };

    /// Pure white.
    pub const WHITE: Colour = Colour {
        red: 0xff,
        green: 0xff,
        blue: 0xff,
    };

    /// The web-colour "silver".
    pub const SILVER: Colour = Colour {
        red: 192,
        green: 192,
        blue: 192,
    };
}
