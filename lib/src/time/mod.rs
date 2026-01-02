//! Types pertaining to [real time](https://en.wikipedia.org/wiki/Time).

mod duration;
mod instant;
mod period;
mod tempo;

pub use duration::Duration;
pub use duration::NonZeroDuration;
pub use instant::Instant;
pub use instant::NonZeroInstant;
pub use period::Period;
pub use tempo::Tempo;
