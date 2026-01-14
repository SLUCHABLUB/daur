//! Items pertaining to [`Offset`].

mod ops;

use crate::ui::Length;
use saturating_cast::SaturatingCast as _;

// TODO: document the not-fully-saturating semantics on overflow.
/// A signed [length](Length).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Offset {
    /// The number of pixels to offset by.
    pixels: i32,
}

impl Offset {
    /// Constructs a positive offset.
    #[must_use]
    pub const fn positive(length: Length) -> Offset {
        Offset {
            pixels: length.pixels as i32,
        }
    }

    /// Constructs a negative offset.
    #[must_use]
    pub const fn negative(length: Length) -> Offset {
        #[expect(clippy::arithmetic_side_effects, reason = "we encapsulate in i64")]
        Offset {
            pixels: -(length.pixels as i32),
        }
    }

    /// 0
    pub const ZERO: Offset = Offset { pixels: 0 };

    /// One pixel.
    pub const PIXEL: Offset = Offset { pixels: 1 };

    /// Returns the absolute value of the offset.
    #[must_use]
    pub fn abs(self) -> Length {
        if self.pixels.is_negative() {
            -self
        } else {
            self
        }
        .rectify()
    }

    /// Convert the offset to a [length](Length).
    ///
    /// Negative values are mapped to 0.
    #[must_use]
    pub fn rectify(self) -> Length {
        Length {
            pixels: self.pixels.saturating_cast(),
        }
    }
}

impl From<Length> for Offset {
    fn from(length: Length) -> Offset {
        Offset::positive(length)
    }
}
