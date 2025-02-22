use crate::measure::Length;
use std::num::NonZeroU16;

const ONE: NonZeroU16 = NonZeroU16::MIN;
const FOUR: NonZeroU16 = ONE.saturating_add(3);

/// A non-zero orthogonal distance between two points
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonZeroLength {
    inner: NonZeroU16,
}

impl NonZeroLength {
    /// The width of a character
    pub const CHAR_WIDTH: NonZeroLength = NonZeroLength::new(ONE);

    /// The height of a character
    pub const CHAR_HEIGHT: NonZeroLength = NonZeroLength::new(ONE);

    /// The default width of a grid cell
    pub const DEFAULT_CELL_WIDTH: NonZeroLength = NonZeroLength::new(FOUR);

    const fn new(value: NonZeroU16) -> NonZeroLength {
        NonZeroLength { inner: value }
    }

    /// Construct a `NonZeroLength` from a `Length` if it is not 0
    #[must_use]
    pub fn from_length(length: Length) -> Option<NonZeroLength> {
        Some(NonZeroLength {
            inner: NonZeroU16::new(length.inner)?,
        })
    }

    pub(super) fn inner(self) -> NonZeroU16 {
        self.inner
    }

    /// Converts `self` to a [`Length`]
    #[must_use]
    pub fn get(self) -> Length {
        Length::new(self.inner.get())
    }
}
