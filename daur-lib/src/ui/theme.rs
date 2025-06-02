use crate::ui::Colour;

/// A colour theme.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub struct Theme {
    /// The colour of the background.
    pub background: Colour,
    /// The colour of the black keys on the piano.
    pub black_key: Colour,
    /// The colour of the white keys on the piano.
    pub white_key: Colour,
}

impl Theme {
    /// Resolves a [`ThemeColour`] to a colour.
    #[must_use]
    pub fn resolve(self, theme_colour: ThemeColour) -> Colour {
        match theme_colour {
            ThemeColour::Background => self.background,
            ThemeColour::BlackKey => self.black_key,
            ThemeColour::WhiteKey => self.white_key,
            ThemeColour::Custom(colour) => colour,
        }
    }
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            background: Colour::gray_scale(0xFF),
            black_key: Colour::gray_scale(0x00),
            white_key: Colour::gray_scale(0xFF),
        }
    }
}

/// A colour that may depend on the theme.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ThemeColour {
    /// The colour of the background.
    Background,
    /// The colour of the black keys on the piano.
    BlackKey,
    /// The colour of the white keys on the piano.
    WhiteKey,

    /// A custom colour.
    Custom(Colour),
}
