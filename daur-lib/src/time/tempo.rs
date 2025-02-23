use ordered_float::NotNan;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::time::Duration;

/// A musical tempo
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Tempo {
    // INVARIANT: is positive
    bpm: NotNan<f64>,
}

impl Tempo {
    /// The duration of a beat at this tempo
    #[must_use]
    pub fn beat_duration(self) -> Duration {
        Duration::from_secs_f64(self.bpm.recip() * 60.0)
    }
}

// TODO: rationale
impl Default for Tempo {
    fn default() -> Self {
        Tempo {
            #[expect(clippy::unwrap_used, reason = "180.0 is not NaN")]
            bpm: NotNan::new(180.0).unwrap(),
        }
    }
}

impl Display for Tempo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.bpm)
    }
}
