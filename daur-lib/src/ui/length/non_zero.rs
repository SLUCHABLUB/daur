use crate::ui::Length;
use std::num::NonZeroU16;

const ONE: NonZeroU16 = NonZeroU16::MIN;
const FOUR: NonZeroU16 = ONE.saturating_add(3);
const SIX: NonZeroU16 = ONE.saturating_add(5);
const TEN: NonZeroU16 = ONE.saturating_add(9);

/// A non-zero orthogonal distance between two points
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonZeroLength {
    inner: NonZeroU16,
}

impl NonZeroLength {
    /// The minimum non-zero horizontal distance.
    pub const X_MINIMUM: Option<NonZeroLength> = Some(NonZeroLength::new(ONE));

    /// The minimum non-zero vertical distance.
    pub const Y_MINIMUM: Option<NonZeroLength> = Some(NonZeroLength::new(ONE));

    /// The default with of a key in the piano roll.
    pub const DEFAULT_KEY_WIDTH: NonZeroLength = NonZeroLength::new(ONE);

    /// The default width of a grid cell
    pub const DEFAULT_CELL_WIDTH: NonZeroLength = NonZeroLength::new(FOUR);

    /// The default depth of a black key on the piano-roll piano
    pub const DEFAULT_BLACK_KEY_DEPTH: NonZeroLength = NonZeroLength::new(SIX);

    /// The default depth of the piano-roll piano
    pub const DEFAULT_PIANO_DEPTH: NonZeroLength = NonZeroLength::new(TEN);

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
