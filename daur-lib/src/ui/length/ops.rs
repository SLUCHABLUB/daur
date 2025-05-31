use crate::ui::{Length, NonZeroLength, Offset};
use crate::{NonZeroRatio, Ratio};
use saturating_cast::SaturatingCast as _;
use std::num::NonZeroU64;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

// --- INFIX OPERATIONS ---

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Self::Output {
        Length {
            pixels: self.pixels.saturating_add(rhs.pixels),
        }
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Length) -> Self::Output {
        Length {
            pixels: self.pixels.saturating_sub(rhs.pixels),
        }
    }
}

impl Mul<Ratio> for Length {
    type Output = Length;

    fn mul(self, rhs: Ratio) -> Self::Output {
        let pixels = (Ratio::from(self.pixels) * rhs).round().saturating_cast();
        Length { pixels }
    }
}

impl Div<NonZeroRatio> for Length {
    type Output = Length;

    fn div(self, rhs: NonZeroRatio) -> Self::Output {
        #![expect(clippy::suspicious_arithmetic_impl, reason = "we take the reciprocal")]
        self * rhs.reciprocal().get()
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl AddAssign for Length {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Length {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<Ratio> for Length {
    fn mul_assign(&mut self, rhs: Ratio) {
        *self = *self * rhs;
    }
}

impl DivAssign<NonZeroRatio> for Length {
    fn div_assign(&mut self, rhs: NonZeroRatio) {
        *self = *self / rhs;
    }
}

// --- STRICTLY INFIX OPERATIONS ---

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

// --- OFFSET OPERATIONS ---

impl Add<Offset> for Length {
    type Output = Length;

    fn add(self, rhs: Offset) -> Self::Output {
        (Offset::positive(self) + rhs).rectify()
    }
}

impl Sub<Offset> for Length {
    type Output = Length;

    fn sub(self, rhs: Offset) -> Self::Output {
        (Offset::positive(self) - rhs).rectify()
    }
}

impl AddAssign<Offset> for Length {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl SubAssign<Offset> for Length {
    fn sub_assign(&mut self, rhs: Offset) {
        *self = *self - rhs;
    }
}
