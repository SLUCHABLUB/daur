//! Items pertaining to [`Quotum2D`].

use crate::view::Quotum;

/// A two dimensional [quotum](Quotum).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Quotum2D {
    /// The [quotum](Quotum) along the x-axis.
    pub x: Quotum,
    /// The [quotum](Quotum) along the y-axis.
    pub y: Quotum,
}

impl Quotum2D {
    /// All remaining space along both axes. See [`Quotum::Remaining`].
    pub const REMAINING: Quotum2D = Quotum2D {
        x: Quotum::Remaining,
        y: Quotum::Remaining,
    };

    /// The minimum allowed size for the view. See [`Quotum::Minimum`].
    pub const MINIMUM: Quotum2D = Quotum2D {
        x: Quotum::Minimum,
        y: Quotum::Remaining,
    };
}
