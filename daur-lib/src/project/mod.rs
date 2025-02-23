mod action;
mod bar;
pub mod change;
pub mod changing;
mod edit;
pub mod manager;
mod ruler;
mod source;

pub use action::Action;

use crate::app;
use crate::clip::Clip;
use crate::key::Key;
use crate::project::changing::Changing;
use crate::project::ruler::Ruler;
use crate::project::source::ProjectSource;
use crate::time::{Instant, Signature, Tempo};
use crate::track::overview::Overview;
use crate::track::Track;
use crate::ui::{Grid, Length};
use crate::widget::button::Button;
use crate::widget::heterogeneous::TwoStack;
use crate::widget::homogenous::Stack;
use crate::widget::text::Text;
use crate::widget::Widget;
use arcstr::{literal, ArcStr};
use ratatui::prelude::Constraint;
use saturating_cast::SaturatingCast as _;
use std::sync::{Arc, Weak};

const ADD_TRACK_LABEL: ArcStr = literal!("+");
const ADD_TRACK_DESCRIPTION: ArcStr = literal!("add track");

#[derive(Clone, Debug, Default)]
pub struct Project {
    pub title: ArcStr,

    pub key: Arc<Changing<Key>>,
    pub time_signature: Arc<Changing<Signature>>,
    // TODO: continuous change
    pub tempo: Arc<Changing<Tempo>>,

    pub tracks: Vec<Arc<Track>>,
}

impl Project {
    pub fn time_signature(&self) -> Arc<Changing<Signature>> {
        Arc::clone(&self.time_signature)
    }

    pub fn tempo(&self) -> Arc<Changing<Tempo>> {
        Arc::clone(&self.tempo)
    }

    pub fn workspace(
        &self,
        track_settings_size: Length,
        grid: Grid,
        overview_offset: Length,
        selected_track_index: usize,
        selected_clip: &Weak<Clip>,
        cursor: Instant,
    ) -> impl Widget {
        let track_count = self.tracks.len().saturating_cast();

        let horizontal_constraints = [track_settings_size.constraint(), Constraint::Fill(1)];
        let ruler_constraints = [Constraint::Max(2), Constraint::Fill(1)];

        // TODO: put something here?
        let empty_space = Text::EMPTY;

        let ruler = Ruler {
            time_signature: self.time_signature(),
            grid,
            offset: overview_offset,
        };
        let ruler_row = TwoStack::horizontal((empty_space, ruler), horizontal_constraints);

        let mut track_settings = Vec::new();
        let mut track_overviews = Vec::new();

        for (index, track) in self.tracks.iter().map(Arc::clone).enumerate() {
            let selected = index == selected_track_index;
            track_settings.push(track.settings(selected, index));
            track_overviews.push(Overview {
                track,
                selected_clip: Weak::clone(selected_clip),
                time_signature: self.time_signature(),
                tempo: self.tempo(),
                grid,
                offset: overview_offset,
                cursor,
                index,
            });
        }

        // A "dummy-track" for the row with the add track button
        track_overviews.push(Overview {
            track: Arc::new(Track::new()),
            selected_clip: Weak::clone(selected_clip),
            time_signature: self.time_signature(),
            tempo: self.tempo(),
            grid,
            offset: overview_offset,
            cursor,
            index: usize::MAX,
        });

        let add_track_button = Button::described(
            ADD_TRACK_LABEL,
            ADD_TRACK_DESCRIPTION,
            app::Action::Project(Action::AddTrack),
        );

        let settings_column = TwoStack::vertical(
            (
                Stack::equidistant_vertical(track_settings),
                add_track_button,
            ),
            [Constraint::Fill(track_count), Constraint::Fill(1)],
        );
        let overview_column = Stack::equidistant_vertical(track_overviews);

        let track_area =
            TwoStack::horizontal((settings_column, overview_column), horizontal_constraints);

        TwoStack::vertical((ruler_row, track_area), ruler_constraints)
    }

    pub fn to_source(&self, sample_rate: u32, cursor: Instant) -> ProjectSource {
        let offset = cursor.to_sample(&self.time_signature, &self.tempo, sample_rate);
        ProjectSource {
            sample_rate,
            tracks: self
                .tracks
                .iter()
                .map(|track| {
                    track.to_source(&self.time_signature, &self.tempo, sample_rate, offset)
                })
                .collect(),
        }
    }
}
