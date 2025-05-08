use crate::Audio;
use crate::real_time::{Duration, Instant};
use rodio::Sink;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct Player {
    sink: Arc<Sink>,
}

impl Player {
    pub(crate) fn is_playing(&self) -> bool {
        !self.sink.is_paused()
    }

    /// Returns the position if audio is playing.
    pub(crate) fn position(&self) -> Option<Instant> {
        self.is_playing().then(|| Instant {
            since_start: Duration::from(self.sink.get_pos()),
        })
    }

    pub(crate) fn pause(&self) -> Option<Instant> {
        let position = self.position();
        self.sink.clear();
        position
    }

    pub(crate) fn play(&self, audio: Audio, from: Instant) {
        self.sink.clear();
        self.sink.append(audio.into_source());

        // `audio::Source::try_seek` always returns `Ok`
        let _ok = self.sink.try_seek(from.since_start.into());

        self.sink.play();
    }
}

impl From<Sink> for Player {
    fn from(sink: Sink) -> Self {
        Player {
            sink: Arc::new(sink),
        }
    }
}
