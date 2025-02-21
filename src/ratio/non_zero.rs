use crate::ratio::Ratio;
use num::rational;
use std::num::{NonZeroU32, NonZeroU8};
use std::ops::{Div, DivAssign};

#[expect(clippy::unwrap_used, reason = "1 is not zero")]
const ONE: NonZeroU32 = NonZeroU32::new(1).unwrap();
#[expect(clippy::unwrap_used, reason = "4 is not zero")]
const FOUR: NonZeroU32 = NonZeroU32::new(4).unwrap();

#[derive(Copy, Clone, Debug)]
pub struct NonZeroRatio {
    inner: rational::Ratio<NonZeroU32>,
}

impl NonZeroRatio {
    pub const QUARTER: NonZeroRatio = NonZeroRatio {
        inner: rational::Ratio::new_raw(ONE, FOUR),
    };

    pub fn new(numerator: NonZeroU32, denominator: NonZeroU32) -> NonZeroRatio {
        // The non-zero types don't implement `num::Integer`.
        // Therefore, `num::rational::Ratio`s thereof cannot be reduced
        // since it requires comparison with zero.

        #[expect(
            clippy::unwrap_used,
            reason = "since the arguments are non-zero, this will be too"
        )]
        NonZeroRatio::from_ratio(Ratio::new(numerator.get(), denominator.get())).unwrap()
    }

    pub fn get(self) -> Ratio {
        let (numerator, denominator) = self.inner.into_raw();
        let inner = rational::Ratio::new_raw(numerator.get(), denominator.get());
        Ratio { inner }
    }

    pub fn from_ratio(ratio: Ratio) -> Option<NonZeroRatio> {
        let (numerator, denominator) = ratio.inner.into_raw();

        let numerator = NonZeroU32::new(numerator)?;
        let denominator = NonZeroU32::new(denominator)?;

        Some(NonZeroRatio {
            inner: rational::Ratio::new_raw(numerator, denominator),
        })
    }
}

impl From<NonZeroU8> for NonZeroRatio {
    fn from(value: NonZeroU8) -> Self {
        NonZeroRatio {
            inner: rational::Ratio::new_raw(NonZeroU32::from(value), ONE),
        }
    }
}

impl Div for NonZeroRatio {
    type Output = NonZeroRatio;

    fn div(self, rhs: Self) -> Self::Output {
        #[expect(
            clippy::unwrap_used,
            reason = "numerator is non-zero, therefore the result will be too"
        )]
        NonZeroRatio::from_ratio(self.get() / rhs).unwrap()
    }
}

impl DivAssign for NonZeroRatio {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
