//! Items pertaining to [`Player`].

use crate::Audio;
use crate::audio::Source;
use crate::time::Duration;
use crate::time::Instant;
use derive_more::Debug;
use rodio::Sink;
use std::sync::Arc;

/// An audio player.
///
/// It holds a handle to an output stream, connected to an audio output device.
#[derive(Clone, Debug)]
pub(crate) struct Player {
    /// The underlying audio sink.
    #[debug(skip)]
    sink: Arc<Sink>,
}

impl Player {
    /// Returns whether audio is currently playing.
    pub(crate) fn is_playing(&self) -> bool {
        !self.sink.is_paused() && !self.sink.empty()
    }

    /// Returns the position if audio is playing or if it has reached the end.
    pub(crate) fn position(&self) -> Option<Instant> {
        (!self.sink.is_paused()).then_some(Instant {
            since_start: Duration::from(self.sink.get_pos()),
        })
    }

    /// Pauses the audio player.
    pub(crate) fn pause(&self) -> Option<Instant> {
        let position = self.position();
        self.sink.clear();
        position
    }

    /// Plays the given audio starting at the given position.
    pub(crate) fn play(&self, audio: Audio, from: Instant) {
        self.sink.clear();
        self.sink.append(Source::new(audio));

        // `audio::Source::try_seek` always returns `Ok`
        let _ok = self.sink.try_seek(from.since_start.into());

        self.sink.play();
    }
}

impl From<Sink> for Player {
    fn from(sink: Sink) -> Player {
        sink.pause();

        Player {
            sink: Arc::new(sink),
        }
    }
}
