use crate::ui::{Length, Offset};
use crate::{NonZeroRatio, Ratio};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        Offset {
            pixels: self.pixels.saturating_add(rhs.pixels),
        }
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

impl Div<NonZeroRatio> for Offset {
    type Output = Offset;

    fn div(self, rhs: NonZeroRatio) -> Offset {
        #![expect(clippy::suspicious_arithmetic_impl, reason = "we take the reciprocal")]
        self * rhs.reciprocal().get()
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl AddAssign for Offset {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Offset {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<Ratio> for Offset {
    fn mul_assign(&mut self, rhs: Ratio) {
        *self = *self * rhs;
    }
}

impl DivAssign<NonZeroRatio> for Offset {
    fn div_assign(&mut self, rhs: NonZeroRatio) {
        *self = *self / rhs;
    }
}

// --- UNARY OPERATIONS ---

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        Offset {
            pixels: self.pixels.saturating_neg(),
        }
    }
}

// TODO: remove

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
