//! Items pertaining to [`Length`].

mod non_zero;
mod ops;

pub use non_zero::NonZeroLength;

use crate::view::Quotum;

/// An orthogonal distance between two points
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Length {
    /// The number of pixels that fit in the length.
    pub pixels: u16,
}

impl Length {
    /// 0
    pub const ZERO: Length = Length { pixels: 0 };

    /// The length of a single pixel.
    pub const PIXEL: Length = Length { pixels: 1 };

    /// Converts the length to a [quotum](Quotum).
    #[must_use]
    pub const fn quotum(self) -> Quotum {
        Quotum::Exact(self)
    }
}
