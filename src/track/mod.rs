mod overview;

use crate::app::settings::OverviewSettings;
use crate::clip::Clip;
use crate::id::Id;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use crate::track::overview::Overview;
use crate::widget::Widget;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::{Block, Paragraph};
use std::collections::BTreeMap;

const PLACEHOLDER_TITLE: &str = "a track";

#[derive(Clone, Debug)]
pub struct Track {
    pub name: String,
    pub clips: BTreeMap<Instant, Clip>,
    pub id: Id<Track>,
}

impl Track {
    pub fn new() -> Track {
        Track {
            name: PLACEHOLDER_TITLE.to_string(),
            clips: BTreeMap::new(),
            id: Id::new(),
        }
    }

    fn block(&self, selected: bool) -> Block {
        let set = if selected { THICK } else { PLAIN };

        Block::bordered().title(self.name.as_str()).border_set(set)
    }

    pub fn settings(&self, selected: bool) -> impl Widget + use<'_> {
        Paragraph::default().block(self.block(selected))
    }

    pub fn overview<'a>(
        &'a self,
        selected_clip: Id<Clip>,
        time_signature: &'a Changing<TimeSignature>,
        tempo: &'a Changing<Tempo>,
        overview_settings: OverviewSettings,
        cursor: Instant,
    ) -> Overview<'a> {
        Overview {
            track: self,
            selected_clip,
            time_signature,
            tempo,
            settings: overview_settings,
            cursor,
        }
    }
}
