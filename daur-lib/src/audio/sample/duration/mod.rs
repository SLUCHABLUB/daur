mod ops;

/// A duration of sample time. A sample count.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Duration {
    /// The number of samples that fit in the duration.
    pub samples: usize,
}

impl Duration {
    /// 0.
    pub const ZERO: Duration = Duration { samples: 0 };

    /// One sample.
    pub const SAMPLE: Duration = Duration { samples: 1 };
}
