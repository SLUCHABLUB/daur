mod non_zero;

pub use non_zero::NonZeroLength;

use crate::Ratio;
use crate::view::Quotum;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// An orthogonal distance between two points
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Length {
    /// The number of pixels that fit in the length.
    pub pixels: u32,
}

impl Length {
    /// 0
    pub const ZERO: Length = Length { pixels: 0 };

    /// The length of a single pixel.
    pub const PIXEL: Length = Length { pixels: 0 };

    /// Converts the length to a [quotum](Quotum).
    #[must_use]
    pub fn quotum(self) -> Quotum {
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
        let pixels = (Ratio::integer(self.pixels) * rhs).round();
        Length { pixels }
    }
}

// TODO: derive
impl Mul<u32> for Length {
    type Output = Length;

    fn mul(mut self, rhs: u32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<u32> for Length {
    fn mul_assign(&mut self, rhs: u32) {
        self.pixels = self.pixels.saturating_mul(rhs);
    }
}

impl Div<NonZeroLength> for Length {
    type Output = Ratio;

    fn div(self, rhs: NonZeroLength) -> Self::Output {
        Ratio::new(self.pixels, rhs.pixels)
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

// TODO: derive
impl DivAssign<NonZeroU32> for Length {
    fn div_assign(&mut self, rhs: NonZeroU32) {
        *self = *self / rhs;
    }
}
