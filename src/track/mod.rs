mod overview;

use crate::app::overview_settings::OverviewSettings;
use crate::clip::Clip;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use crate::track::overview::Overview;
use crate::widget::Widget;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::{Block, Paragraph};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct Track {
    pub name: String,
    pub clips: BTreeMap<Instant, Clip>,
}

impl Track {
    fn block(&self, selected: bool) -> Block {
        let set = if selected { THICK } else { PLAIN };

        Block::bordered().title(self.name.as_str()).border_set(set)
    }

    pub fn settings(&self, selected: bool) -> impl Widget + use<'_> {
        Paragraph::default().block(self.block(selected))
    }

    pub fn overview<'a>(
        &'a self,
        selected_clip: Option<usize>,
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
