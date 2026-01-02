use crate::ui::Colour;

/// A colour theme.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub struct Theme {
    /// The colour to use for [`ThemeColour::Background`].
    pub background: Colour,
    /// The colour to use for [`ThemeColour::BlackKey`].
    pub black_key: Colour,
    /// The colour to use for [`ThemeColour::WhiteKey`].
    pub white_key: Colour,
    /// The colour to use for [`ThemeColour::PianoRollBackground`].
    /// Defaults to [`background`](Theme::background).
    pub piano_roll_background: Option<Colour>,
    /// The colour to use for [`ThemeColour::AlternatePianoRollBackground`].
    /// Defaults to [`piano_roll_background`](Theme::piano_roll_background).
    pub alternate_piano_roll_background: Option<Colour>,
}

impl Theme {
    /// Resolves a [`ThemeColour`] to a colour.
    #[must_use]
    pub fn resolve(self, theme_colour: ThemeColour) -> Colour {
        match theme_colour {
            ThemeColour::Background => self.background,
            ThemeColour::BlackKey => self.black_key,
            ThemeColour::WhiteKey => self.white_key,
            ThemeColour::PianoRollBackground => {
                self.piano_roll_background.unwrap_or(self.background)
            }
            ThemeColour::AlternatePianoRollBackground => self
                .alternate_piano_roll_background
                .unwrap_or(self.resolve(ThemeColour::Background)),
            ThemeColour::Custom(colour) => colour,
        }
    }
}

/// A minimal dark colour theme.
impl Default for Theme {
    fn default() -> Theme {
        Theme {
            background: Colour::BLACK,
            black_key: Colour::BLACK,
            white_key: Colour::WHITE,
            piano_roll_background: None,
            alternate_piano_roll_background: Some(Colour::SILVER),
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

    /// The main colour of the pianoroll's background.
    PianoRollBackground,
    /// The colour used for every other pitch-row in the pianoroll's background.
    AlternatePianoRollBackground,

    /// A custom colour.
    Custom(Colour),
}
