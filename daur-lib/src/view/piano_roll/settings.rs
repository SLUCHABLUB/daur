use crate::ui::{Length, NonZeroLength, Offset};

/// Settings for the piano roll.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Settings {
    /// How far along the piano roll is scrolled.
    pub x_offset: Length,
    /// How far from A4 the piano roll is scrolled.
    pub y_offset: Offset,
    /// The height of the piano roll itself.
    /// Or [`None`] if the piano roll is closed.
    pub height: Option<NonZeroLength>,

    /// The width of the keys
    pub key_width: NonZeroLength,
    /// The full depth of the white keys
    pub piano_depth: NonZeroLength,
    /// The depth of the black keys
    pub black_key_depth: NonZeroLength,
}
