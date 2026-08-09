//! Items pertaining to [`Player`].

use crate::Audio;
use crate::audio::Source;
use crate::time::Duration;
use crate::time::Instant;
use derive_more::Debug;
use std::sync::Arc;

/// An audio player.
///
/// # Semantics
///
/// The player is either playing or not.
/// If the player reaches the end, it will stop.
#[derive(Clone, Debug)]
pub(crate) struct Player {
    /// The underlying audio sink.
    #[debug(skip)]
    inner: Arc<rodio::Player>,
}

impl Player {
    /// Returns whether audio is currently playing.
    pub(crate) fn is_playing(&self) -> bool {
        !self.inner.is_paused() && !self.inner.empty()
    }

    // TODO: This signature is confusing.
    /// Returns the position if audio is playing or if it has reached the end.
    pub(crate) fn position(&self) -> Option<Instant> {
        (!self.inner.is_paused()).then_some(Instant {
            since_start: Duration::from(self.inner.get_pos()),
        })
    }

    /// Pauses the audio player.
    pub(crate) fn pause(&self) -> Option<Instant> {
        let position = self.position();
        self.inner.clear();
        position
    }

    /// Plays the given audio starting at the given position.
    pub(crate) fn play(&self, audio: Audio, from: Instant) {
        self.inner.clear();
        self.inner.append(Source::new(audio));

        // `audio::Source::try_seek` always returns `Ok`
        let _ok = self.inner.try_seek(from.since_start.into());

        self.inner.play();
    }
}

impl From<rodio::Player> for Player {
    fn from(inner: rodio::Player) -> Player {
        inner.pause();

        Player {
            inner: Arc::new(inner),
        }
    }
}
