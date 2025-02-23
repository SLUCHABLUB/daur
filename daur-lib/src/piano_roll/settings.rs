use crate::ui::{Length, NonZeroLength};

/// Settings pertaining to the piano roll
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PianoRollSettings {
    /// The height of the piano roll itself
    pub height: Length,

    /// The width of the keys
    pub key_width: NonZeroLength,
    /// The full depth of the white keys
    pub piano_depth: NonZeroLength,
    /// The depth of the black keys
    pub black_key_depth: NonZeroLength,
}

impl Default for PianoRollSettings {
    fn default() -> Self {
        PianoRollSettings {
            height: Length::ZERO,
            key_width: NonZeroLength::CHAR_HEIGHT,
            piano_depth: NonZeroLength::DEFAULT_PIANO_DEPTH,
            black_key_depth: NonZeroLength::DEFAULT_BLACK_KEY_DEPTH,
        }
    }
}
