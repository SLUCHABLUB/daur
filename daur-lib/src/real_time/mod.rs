//! Types pertaining to [real time](https://en.wikipedia.org/wiki/Time).

mod duration;
mod instant;
mod tempo;

pub use duration::{Duration, NonZeroDuration};
pub use instant::{Instant, NonZeroInstant};
pub use tempo::Tempo;
