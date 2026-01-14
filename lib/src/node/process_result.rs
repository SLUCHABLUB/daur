//! Items pertaining to [`ProcessResult`].

use crate::Audio;

/// The result of processing a slice of a clip.
#[must_use]
pub struct ProcessResult {
    /// The output audio.
    pub audio: Audio,
    /// Whether the processor has more audio to output, even if no new input is received.
    pub should_continue: bool,
}
