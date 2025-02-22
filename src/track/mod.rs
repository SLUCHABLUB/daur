pub mod overview;
mod settings;
mod source;

pub use source::TrackSource;
use std::collections::BTreeMap;

use crate::clip::Clip;
use crate::project::changing::Changing;
use crate::time::tempo::Tempo;
use crate::time::{Instant, TimeSignature};
use crate::track::settings::Settings;
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

    pub fn to_source(
        &self,
        time_signature: &Changing<TimeSignature>,
        tempo: &Changing<Tempo>,
        sample_rate: u32,
        offset: usize,
    ) -> TrackSource {
        TrackSource::new(
            sample_rate,
            self.clips
                .iter()
                .map(|(start, clip)| {
                    let start = start.to_sample(time_signature, tempo, sample_rate);
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
