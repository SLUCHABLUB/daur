mod non_zero;

pub use non_zero::NonZeroLength;

use crate::Ratio;
use ratatui::layout::{Constraint, Spacing};
use saturating_cast::SaturatingCast as _;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

/// An orthogonal distance between two points
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Length {
    inner: u16,
}

impl Length {
    pub(super) const fn new(value: u16) -> Length {
        Length { inner: value }
    }

    pub(super) const fn inner(self) -> u16 {
        self.inner
    }

    /// 0
    pub const ZERO: Length = Length::new(0);

    /// Double the border-thickness of a bordered view
    pub const DOUBLE_BORDER: Length = Length::new(2);

    /// The width of the musical cursor
    pub const CURSOR_WIDTH: Length = Length::CHAR_WIDTH;

    /// The width of a character
    pub const CHAR_WIDTH: Length = Length::new(1);

    /// The height of a character
    pub const CHAR_HEIGHT: Length = Length::new(1);

    /// The minimum height of the project bar
    pub const PROJECT_BAR_MINIMUM: Length = Length::new(5);

    /// The default width of the track-settings sidebar
    pub const TRACK_SETTINGS_DEFAULT: Length = Length::new(20);

    pub(crate) const MAX: Length = Length::new(u16::MAX);

    /// Returns the height of the string
    #[must_use]
    pub fn string_height(string: &str) -> Self {
        let length = string.lines().count();
        let length = length.saturating_cast();
        Length::new(length)
    }

    /// Returns the width of the string
    #[must_use]
    pub fn string_width(string: &str) -> Self {
        // TODO: use graphemes
        let length = string.chars().count();
        let length = length.saturating_cast();
        Length::new(length)
    }

    /// Converts `self` to a `Constraint`
    #[must_use]
    pub fn constraint(self) -> Constraint {
        Constraint::Length(self.inner)
    }
}

impl From<Length> for Spacing {
    fn from(length: Length) -> Self {
        Spacing::Space(length.inner)
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Self::Output {
        Length::new(self.inner.saturating_add(rhs.inner))
    }
}

impl AddAssign for Length {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Length) -> Self::Output {
        Length::new(self.inner.saturating_sub(rhs.inner))
    }
}

impl Mul<Ratio> for Length {
    type Output = Length;

    fn mul(self, rhs: Ratio) -> Self::Output {
        let length = (Ratio::integer(self.inner.into()) * rhs).round();
        let length = length.saturating_cast();
        Length::new(length)
    }
}

impl Mul<u32> for Length {
    type Output = Length;

    fn mul(self, rhs: u32) -> Self::Output {
        self * Ratio::integer(rhs)
    }
}

impl Mul<usize> for Length {
    type Output = Length;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs.saturating_cast::<u32>()
    }
}

impl Div<NonZeroLength> for Length {
    type Output = Ratio;

    fn div(self, rhs: NonZeroLength) -> Self::Output {
        let denominator = NonZeroU32::from(rhs.inner());
        Ratio::new(u32::from(self.inner), denominator)
    }
}

impl Div<NonZeroU32> for Length {
    type Output = Length;

    fn div(self, rhs: NonZeroU32) -> Self::Output {
        #![expect(
            clippy::suspicious_arithmetic_impl,
            reason = "we multiply by the reciprocal"
        )]
        self * Ratio::new(1, rhs)
    }
}
