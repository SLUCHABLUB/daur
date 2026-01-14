//! Items pertaining to [`Alignment`].

/// How something should be aligned
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Alignment {
    /// Alignment to the top-left
    TopLeft,
    /// Alignment to the top (centred horizontally)
    Top,
    /// Alignment to the top-right
    TopRight,
    /// Alignment to the left (centred vertically)
    Left,
    /// Centred
    Centre,
    /// Alignment to the right (centred vertically)
    Right,
    /// Alignment to the bottom-left
    BottomLeft,
    /// Alignment to the bottom (centred horizontally)
    Bottom,
    /// Alignment to the bottom-right
    BottomRight,
}
