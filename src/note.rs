use crate::pitch::Pitch;
use crate::time::duration::Duration;

// TODO: pitch-bends?
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Duration,
    // TODO: articulation
}
