pub mod overview;
mod settings;
mod source;

pub use source::TrackSource;
use std::collections::BTreeMap;

use crate::time::{Instant, Mapping};
use crate::track::settings::Settings;
use crate::Clip;
use arcstr::{literal, ArcStr};
use std::sync::Arc;

const DEFAULT_TITLE: ArcStr = literal!("a track");

#[derive(Clone, Debug)]
pub struct Track {
    pub name: ArcStr,
    pub clips: BTreeMap<Instant, Arc<Clip>>,
}

impl Track {
    pub fn new() -> Track {
        Track {
            name: DEFAULT_TITLE,
            clips: BTreeMap::new(),
        }
    }

    pub fn settings(self: &Arc<Self>, selected: bool, index: usize) -> Settings {
        Settings {
            track: Arc::clone(self),
            selected,
            index,
        }
    }

    pub fn to_source(&self, mapping: &Mapping, sample_rate: u32, offset: usize) -> TrackSource {
        TrackSource::new(
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
