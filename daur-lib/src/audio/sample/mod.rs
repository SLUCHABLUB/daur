//! Items pertaining to [`Sample`].

mod duration;
mod instant;
mod period;
mod rate;

pub use duration::Duration;
pub use instant::Instant;
pub use period::Period;
pub use rate::{Rate, ZeroRateError};

use bytemuck::NoUninit;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, AddAssign};

/// A 32-bit float sample
#[derive(Copy, Clone, PartialEq, Default, NoUninit)]
#[repr(transparent)]
#[cfg_attr(doc, doc(hidden))]
pub struct Sample {
    // INVARIANT: this is on the interval [-1, 1].
    inner: f32,
}

impl Sample {
    /// 0
    pub const ZERO: Sample = Sample { inner: 0.0 };

    /// Constructs a new sample from a float.
    ///
    /// If it is not in range, it is clamped.
    #[must_use]
    pub const fn new(value: f32) -> Sample {
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
        Sample {
            inner: f32::midpoint(self.inner, other.inner),
        }
    }

    /// Converts the float sample to an `f32`.
    #[must_use]
    pub const fn to_f32(self) -> f32 {
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

    fn add(self, rhs: Sample) -> Sample {
        Sample::new(self.inner + rhs.inner)
    }
}

impl AddAssign for Sample {
    fn add_assign(&mut self, rhs: Sample) {
        *self = *self + rhs;
    }
}
