//! Types pertaining to [real time](https://en.wikipedia.org/wiki/Time).

mod duration;
mod instant;

pub use duration::{Duration, NonZeroDuration};
pub use instant::{Instant, NonZeroInstant};
