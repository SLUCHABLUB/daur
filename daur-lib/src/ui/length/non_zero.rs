use crate::ui::Length;
use std::num::NonZeroU16;

/// A non-zero orthogonal distance between two points
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NonZeroLength {
    /// The number of pixels that fit in the length.
    pub pixels: NonZeroU16,
}

impl NonZeroLength {
    /// Construct a length from a [zeroable one](Length) if it is not 0.
    #[must_use]
    pub fn from_length(length: Length) -> Option<NonZeroLength> {
        Some(NonZeroLength {
            pixels: NonZeroU16::new(length.pixels)?,
        })
    }

    /// Converts the length to a [zeroable one](Length).
    #[must_use]
    pub const fn get(self) -> Length {
        Length {
            pixels: self.pixels.get(),
        }
    }
}
