pub mod overview;
pub mod source;

use crate::app::settings::OverviewSettings;
use crate::clip::Clip;
use crate::locked_tree::LockedTree;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use crate::track::overview::Overview;
use crate::track::source::TrackSource;
use crate::widget::Widget;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::{Block, Paragraph};
use std::sync::{Arc, Weak};

const PLACEHOLDER_TITLE: &str = "a track";

#[derive(Clone)]
pub struct Track {
    pub name: String,
    pub clips: LockedTree<Instant, Arc<Clip>>,
}

impl Track {
    pub fn new() -> Track {
        Track {
            name: PLACEHOLDER_TITLE.to_string(),
            clips: LockedTree::new(),
        }
    }

    fn block(&self, selected: bool) -> Block<'static> {
        let set = if selected { THICK } else { PLAIN };

        Block::bordered().title(self.name.clone()).border_set(set)
    }

    pub fn settings(&self, selected: bool) -> impl Widget {
        Paragraph::default().block(self.block(selected))
    }

    pub fn overview<'a>(
        self: &Arc<Self>,
        selected_clip: Weak<Clip>,
        time_signature: &'a Changing<TimeSignature>,
        tempo: &'a Changing<Tempo>,
        overview_settings: OverviewSettings,
        cursor: Instant,
    ) -> Overview<'a> {
        Overview {
            track: Arc::clone(self),
            selected_clip,
            time_signature,
            tempo,
            settings: overview_settings,
            cursor,
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
                .map(|start, clip| {
                    let start = start.to_sample(time_signature, tempo, sample_rate);
                    let mut clip_offset = 0;

                    if start < offset {
                        clip_offset = offset - start;
                    }

                    (start, clip.to_source(clip_offset))
                })
                .collect(),
            offset,
        )
    }
}
