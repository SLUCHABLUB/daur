/// An 8-bit per channel (opaque) colour.
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
    /// Constructs a gray-scale colour.
    #[must_use]
    pub const fn gray_scale(lightness: u8) -> Colour {
        Colour {
            red: lightness,
            green: lightness,
            blue: lightness,
        }
    }

    // TODO: move to `Theme`
    pub(crate) const BACKGROUND: Colour = Colour::gray_scale(0);

    // TODO: move to `Theme`
    /// Black.
    pub const BLACK_KEY: Colour = Colour::gray_scale(0);

    // TODO: move to `Theme`
    /// White.
    pub const WHITE_KEY: Colour = Colour::gray_scale(u8::MAX);
}
