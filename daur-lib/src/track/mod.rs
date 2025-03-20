//! Types relating to [`Track`].

mod overview;
mod settings;
mod source;

pub use overview::{open_import_audio_popup, overview};
pub use settings::settings;
pub use source::Source;

use crate::time::{Instant, Mapping};
use crate::Clip;
use arcstr::{literal, ArcStr};
use std::collections::BTreeMap;
use std::sync::Arc;

const DEFAULT_TITLE: ArcStr = literal!("a track");

/// A musical track.
#[derive(Clone, Debug)]
pub struct Track {
    /// The name of the track.
    pub name: ArcStr,
    /// The clips in the track.
    pub clips: BTreeMap<Instant, Arc<Clip>>,
}

impl Track {
    /// Constructs a new, empty, track.
    #[must_use]
    pub fn new() -> Track {
        Track {
            name: DEFAULT_TITLE,
            clips: BTreeMap::new(),
        }
    }

    /// Returns the audio source for the track.
    #[must_use]
    pub fn to_source(&self, mapping: &Mapping, sample_rate: u32, offset: usize) -> Source {
        Source::new(
            sample_rate,
            self.clips
                .iter()
                .map(|(start, clip)| {
                    let start = start.to_sample(mapping, sample_rate);
                    let mut clip_offset = 0;

                    if start < offset {
                        clip_offset = offset.saturating_sub(start);
                    }

                    (start, clip.to_source(clip_offset))
                })
                .collect(),
            offset,
        )
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}
