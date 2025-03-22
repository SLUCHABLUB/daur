use crate::ui::Length;
use saturating_cast::SaturatingCast as _;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

/// A signed [length](Length).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Offset {
    inner: i32,
}

impl Offset {
    const fn new(value: i32) -> Offset {
        Offset { inner: value }
    }

    /// Constructs a positive offset.
    #[must_use]
    pub const fn positive(length: Length) -> Offset {
        Offset {
            inner: length.inner() as i32,
        }
    }

    /// Constructs a negative offset.
    #[must_use]
    pub const fn negative(length: Length) -> Offset {
        #[expect(clippy::arithmetic_side_effects, reason = "we encapsulate in i32")]
        Offset {
            inner: -(length.inner() as i32),
        }
    }

    /// 0
    pub const ZERO: Offset = Offset::new(0);

    /// Returns the absolute value of the offset.
    #[must_use]
    pub fn abs(self) -> Length {
        if self.inner.is_negative() {
            -self
        } else {
            self
        }
        .saturate()
    }

    /// Convert the offset to a [length](Length) by saturating
    #[must_use]
    pub fn saturate(self) -> Length {
        Length::new(self.inner.saturating_cast())
    }
}

impl From<Length> for Offset {
    fn from(length: Length) -> Self {
        Offset::new(i32::from(length.inner()))
    }
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        Offset {
            inner: self.inner.saturating_add(rhs.inner),
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
            inner: self.inner.saturating_sub(rhs.inner),
        }
    }
}

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        Offset {
            inner: self.inner.saturating_neg(),
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

impl Mul<i32> for Offset {
    type Output = Offset;

    fn mul(self, rhs: i32) -> Self::Output {
        Offset {
            inner: self.inner.saturating_mul(rhs),
        }
    }
}

impl Mul<usize> for Offset {
    type Output = Offset;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs.saturating_cast::<i32>()
    }
}

impl AddAssign<Length> for Offset {
    fn add_assign(&mut self, rhs: Length) {
        *self = *self + rhs;
    }
}

impl AddAssign<Offset> for Length {
    fn add_assign(&mut self, rhs: Offset) {
        *self = (rhs + *self).saturate();
    }
}

impl SubAssign<Length> for Offset {
    fn sub_assign(&mut self, rhs: Length) {
        *self = *self - rhs;
    }
}

impl SubAssign<Offset> for Length {
    fn sub_assign(&mut self, rhs: Offset) {
        *self = ((-rhs) + *self).saturate();
    }
}
