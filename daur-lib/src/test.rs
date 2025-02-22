use crate::time::{Duration, Instant, NonZeroDuration, NonZeroInstant};
use crate::{NonZeroRatio, Ratio};

// Important for safety that these all have the same size
#[test]
fn ratio_size() {
    const SIZE: usize = 8;

    assert_eq!(size_of::<Ratio>(), SIZE);
    assert_eq!(size_of::<NonZeroRatio>(), SIZE);
    assert_eq!(size_of::<Duration>(), SIZE);
    assert_eq!(size_of::<NonZeroDuration>(), SIZE);
    assert_eq!(size_of::<Instant>(), SIZE);
    assert_eq!(size_of::<NonZeroInstant>(), SIZE);
}
