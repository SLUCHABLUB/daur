mod non_zero;

pub use non_zero::NonZeroLength;

use crate::Ratio;
use crate::ui::Offset;
use crate::view::Quotum;
use saturating_cast::SaturatingCast as _;
use std::num::NonZeroU64;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, Sub, SubAssign};

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

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Self::Output {
        Length {
            pixels: self.pixels.saturating_add(rhs.pixels),
        }
    }
}

// TODO: derive
impl AddAssign for Length {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// TODO: derive
impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Length) -> Self::Output {
        Length {
            pixels: self.pixels.saturating_sub(rhs.pixels),
        }
    }
}

// TODO: derive
impl SubAssign for Length {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<Ratio> for Length {
    type Output = Length;

    fn mul(self, rhs: Ratio) -> Self::Output {
        let pixels = (Ratio::from(self.pixels) * rhs).round().saturating_cast();
        Length { pixels }
    }
}

impl MulAssign<Ratio> for Length {
    fn mul_assign(&mut self, rhs: Ratio) {
        *self = *self * rhs;
    }
}

impl Div<NonZeroLength> for Length {
    type Output = Ratio;

    fn div(self, rhs: NonZeroLength) -> Ratio {
        Ratio::new(u64::from(self.pixels), NonZeroU64::from(rhs.pixels))
    }
}

impl Rem<NonZeroLength> for Length {
    type Output = Length;

    fn rem(self, rhs: NonZeroLength) -> Length {
        Length {
            pixels: self.pixels % rhs.pixels,
        }
    }
}

impl Add<Offset> for Length {
    type Output = Length;

    fn add(self, rhs: Offset) -> Self::Output {
        (Offset::positive(self) + rhs).rectify()
    }
}

// TODO: derive
impl AddAssign<Offset> for Length {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl Sub<Offset> for Length {
    type Output = Length;

    fn sub(self, rhs: Offset) -> Self::Output {
        (Offset::positive(self) - rhs).rectify()
    }
}

// TODO: derive
impl SubAssign<Offset> for Length {
    fn sub_assign(&mut self, rhs: Offset) {
        *self = *self - rhs;
    }
}
