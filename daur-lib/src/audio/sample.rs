use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, AddAssign, Div};

/// 2^31
const I32_ABS_MAX: f64 = i32::MAX as f64 + 1.0;

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

    /// Calculates the average of two samples.
    #[must_use]
    pub const fn average(self, other: Sample) -> Sample {
        Sample::new((self.inner + other.inner) / 2.0)
    }

    /// Losslessly constructs a new float sample from a 32-bit integral sample.
    #[must_use]
    pub fn from_i32(sample: i32) -> Sample {
        Sample::new(f64::from(sample) / I32_ABS_MAX)
    }

    /// Converts the float sample to a 32-bit integral one.
    ///
    /// This conversion may be lossy.
    #[must_use]
    pub fn to_i32(self) -> i32 {
        #![expect(clippy::cast_possible_truncation, reason = "lossy conversion")]
        (self.inner * I32_ABS_MAX) as i32
    }

    /// Losslessly constructs a new float sample from a 32-bit one.
    #[must_use]
    pub const fn from_f32(value: f32) -> Sample {
        Sample::new(value as f64)
    }

    /// Converts the float sample to a 32-bit one.
    ///
    /// This conversion may be lossy.
    #[must_use]
    pub const fn to_f32(self) -> f32 {
        #![expect(clippy::cast_possible_truncation, reason = "lossy conversion")]
        self.inner as f32
    }

    /// Converts the float sample to an `f64`.
    #[must_use]
    pub const fn to_f64(self) -> f64 {
        self.inner
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

#[cfg(test)]
#[test]
fn test_losslessness() {
    #![allow(clippy::unreadable_literal, reason = "the numbers are not important")]

    use std::hint::black_box;

    fn back_and_fourth(sample: i32) -> i32 {
        let sample = Sample::from_i32(sample);
        black_box(sample).to_i32()
    }

    fn assert_lossless(sample: i32) {
        assert_eq!(sample, back_and_fourth(sample));
    }

    assert_lossless(0);
    assert_lossless(i32::MIN);
    assert_lossless(i32::MAX);

    // some random samples
    assert_lossless(937306913);
    assert_lossless(414571583);
    assert_lossless(-1656970901);
    assert_lossless(1732022799);
    assert_lossless(-529550899);
    assert_lossless(-2142237014);
    assert_lossless(2038762849);
    assert_lossless(1339135784);
    assert_lossless(-1988583575);
    assert_lossless(-1579592110);
}
