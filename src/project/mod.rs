mod bar;
pub mod change;
pub mod changing;
mod ruler;
mod source;

use crate::app::action::Action;
use crate::app::OverviewSettings;
use crate::clip::Clip;
use crate::key::Key;
use crate::locked_vec::LockedVec;
use crate::project::changing::Changing;
use crate::project::ruler::Ruler;
use crate::project::source::ProjectSource;
use crate::time::instant::Instant;
use crate::time::tempo::Tempo;
use crate::time::TimeSignature;
use crate::track::Track;
use crate::widget::button::Button;
use crate::widget::heterogeneous_stack::TwoStack;
use crate::widget::homogenous_stack::HomogenousStack;
use crate::widget::Widget;
use ratatui::prelude::Constraint;
use ratatui::widgets::Clear;
use saturating_cast::SaturatingCast;
use std::borrow::Cow;
use std::sync::{Arc, Weak};

const PLAY: &str = "\u{25B6}";
const PAUSE: &str = "\u{23F8}";

const PLAY_DESCRIPTION: &str = "play";
const PAUSE_DESCRIPTION: &str = "pause";

#[derive(Clone, Default)]
pub struct Project {
    pub title: String,

    pub key: Changing<Key>,
    pub time_signature: Changing<TimeSignature>,
    // TODO: continuous change
    pub tempo: Changing<Tempo>,

    pub tracks: LockedVec<Arc<Track>>,
}

impl Project {
    pub fn workspace(
        &self,
        track_settings_size: u16,
        overview_settings: OverviewSettings,
        selected_track_index: usize,
        selected_clip: &Weak<Clip>,
        cursor: Instant,
    ) -> impl Widget + use<'_> {
        let track_count = self.tracks.len().saturating_cast();

        let horizontal_constraints = [Constraint::Length(track_settings_size), Constraint::Fill(1)];
        let ruler_constraints = [Constraint::Max(2), Constraint::Fill(1)];

        let ruler = Ruler {
            time_signature: &self.time_signature,
            overview_settings,
        };
        let ruler_row = TwoStack::horizontal((Clear, ruler), horizontal_constraints);

        let mut track_settings = Vec::new();
        let mut track_overviews = Vec::new();

        self.tracks.map_enumerated(|index, track| {
            let selected = index == selected_track_index;
            track_settings.push(track.settings(selected));
            track_overviews.push(track.overview(
                selected_clip,
                &self.time_signature,
                &self.tempo,
                overview_settings,
                cursor,
            ));
        });

        // A "dummy-track" for the row with the add track button
        track_overviews.push(Track::new().overview(
            selected_clip,
            &self.time_signature,
            &self.tempo,
            overview_settings,
            cursor,
        ));

        let add_track_button = Button::new(Cow::Borrowed("+"), Action::AddTrack)
            .description(Cow::Borrowed("add track"))
            .bordered();

        let settings_column = TwoStack::vertical(
            (
                HomogenousStack::equidistant_vertical(track_settings),
                add_track_button,
            ),
            [Constraint::Fill(track_count), Constraint::Fill(1)],
        );
        let overview_column = HomogenousStack::equidistant_vertical(track_overviews);

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
                .map(|track| {
                    track.to_source(&self.time_signature, &self.tempo, sample_rate, offset)
                })
                .collect(),
        }
    }
}
