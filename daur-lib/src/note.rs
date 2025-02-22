use crate::pitch::Pitch;
use crate::time::Duration;

// TODO: pitch-bends?
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Duration,
    // TODO: articulation
}
