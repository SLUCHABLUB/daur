use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, AddAssign, Div};

/// A 64-bit float sample
#[derive(Copy, Clone, PartialEq, Default)]
#[repr(transparent)]
pub struct Sample {
    // INVARIANT: this is on the interval [-1, 1].
    inner: f64,
}

impl Sample {
    /// 0
    pub const ZERO: Sample = Sample { inner: 0.0 };

    /// Constructs a new sample from a float.
    ///
    /// If it is not in range, it is clamped.
    #[must_use]
    pub const fn new(value: f64) -> Sample {
        if value.is_nan() {
            Sample::ZERO
        } else {
            Sample {
                inner: value.clamp(-1.0, 1.0),
            }
        }
    }

    /// Losslessly constructs a new float sample from a 32-bit integral sample.
    // TODO: test
    #[must_use]
    pub fn from_i32(sample: i32) -> Sample {
        Sample::new(f64::from(sample) / (f64::from(i32::MAX) + 1.0))
    }

    /// Constructs a new float sample from a 32-bit one.
    #[must_use]
    pub const fn from_f32(value: f32) -> Sample {
        Sample::new(value as f64)
    }

    /// Converts the float sample to a 32-bit one.
    ///
    /// This conversion is lossy.
    #[must_use]
    pub const fn to_f32(self) -> f32 {
        #![expect(clippy::cast_possible_truncation, reason = "lossy conversion")]
        self.inner as f32
    }
}

// This is fine since the internal float is not NaN
impl Eq for Sample {}

impl Debug for Sample {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Add for Sample {
    type Output = Sample;

    fn add(self, rhs: Self) -> Sample {
        Sample::new(self.inner + rhs.inner)
    }
}

impl AddAssign for Sample {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Div<i32> for Sample {
    type Output = Sample;

    fn div(self, rhs: i32) -> Sample {
        Sample::new(self.inner / f64::from(rhs))
    }
}
