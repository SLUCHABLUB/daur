use crate::Ratio;
use crate::ui::{Length, NonZeroLength};
use saturating_cast::SaturatingCast as _;
use std::num::NonZeroI32;
use std::ops::{Add, AddAssign, Mul, Neg, Rem, Sub, SubAssign};

// TODO: document the not-fully-saturating semantics on overflow.
/// A signed [length](Length).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Offset {
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
    fn from(length: Length) -> Self {
        Offset::positive(length)
    }
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        Offset {
            pixels: self.pixels.saturating_add(rhs.pixels),
        }
    }
}

impl AddAssign for Offset {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Offset {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Self::Output {
        Offset {
            pixels: self.pixels.saturating_sub(rhs.pixels),
        }
    }
}

impl SubAssign for Offset {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<Ratio> for Offset {
    type Output = Offset;

    fn mul(self, rhs: Ratio) -> Self::Output {
        let length = self.abs() * rhs;

        if self.pixels.is_negative() {
            Offset::negative(length)
        } else {
            Offset::positive(length)
        }
    }
}

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        Offset {
            pixels: self.pixels.saturating_neg(),
        }
    }
}

impl Add<Length> for Offset {
    type Output = Offset;

    fn add(self, rhs: Length) -> Self::Output {
        self + Offset::from(rhs)
    }
}

impl Sub<Length> for Offset {
    type Output = Offset;

    fn sub(self, rhs: Length) -> Self::Output {
        self - Offset::from(rhs)
    }
}

impl AddAssign<Length> for Offset {
    fn add_assign(&mut self, rhs: Length) {
        *self = *self + rhs;
    }
}

impl SubAssign<Length> for Offset {
    fn sub_assign(&mut self, rhs: Length) {
        *self = *self - rhs;
    }
}

impl Rem<NonZeroLength> for Offset {
    type Output = Length;

    fn rem(self, rhs: NonZeroLength) -> Length {
        Length {
            pixels: self
                .pixels
                .rem_euclid(NonZeroI32::from(rhs.pixels).get())
                .saturating_cast(),
        }
    }
}
