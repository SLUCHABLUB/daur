use crate::Audio;

#[must_use]
pub struct ProcessResult {
    /// The output audio.
    pub audio: Audio,
    /// Whether the processor has more audio to output, even if no new input is received.
    pub should_continue: bool,
}
